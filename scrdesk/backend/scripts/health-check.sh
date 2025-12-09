#!/bin/bash

##############################################################################
# ScrDesk PRO Enterprise - Health Check Script
#
# This script checks the health of all microservices and infrastructure
# components. It can be run manually or scheduled via cron.
#
# Usage:
#   ./health-check.sh              # Check all services
#   ./health-check.sh --verbose    # Detailed output
#   ./health-check.sh --json       # JSON output for monitoring tools
#   ./health-check.sh --alert      # Send alerts on failures
#
# Exit codes:
#   0 - All services healthy
#   1 - One or more services unhealthy
#   2 - Critical infrastructure failure
##############################################################################

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_FILE="/var/log/scrdesk/health-check.log"
ALERT_EMAIL="${ALERT_EMAIL:-admin@example.com}"
ALERT_WEBHOOK="${ALERT_WEBHOOK:-}"

# Flags
VERBOSE=false
JSON_OUTPUT=false
SEND_ALERTS=false
FAILED_SERVICES=()
HEALTHY_SERVICES=()

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --verbose|-v)
            VERBOSE=true
            shift
            ;;
        --json|-j)
            JSON_OUTPUT=true
            shift
            ;;
        --alert|-a)
            SEND_ALERTS=true
            shift
            ;;
        --help|-h)
            echo "Usage: $0 [--verbose] [--json] [--alert]"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Logging function
log() {
    local level=$1
    shift
    local message="$@"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    if [ "$JSON_OUTPUT" != "true" ]; then
        case $level in
            INFO)
                echo -e "${BLUE}[INFO]${NC} $message"
                ;;
            SUCCESS)
                echo -e "${GREEN}[OK]${NC} $message"
                ;;
            WARNING)
                echo -e "${YELLOW}[WARN]${NC} $message"
                ;;
            ERROR)
                echo -e "${RED}[ERROR]${NC} $message"
                ;;
        esac
    fi

    # Log to file if exists
    if [ -w "$LOG_FILE" ] || [ -w "$(dirname "$LOG_FILE")" ]; then
        echo "[$timestamp] [$level] $message" >> "$LOG_FILE"
    fi
}

# Check if Docker is running
check_docker() {
    log INFO "Checking Docker daemon..."

    if ! command -v docker &> /dev/null; then
        log ERROR "Docker is not installed"
        return 2
    fi

    if ! docker info &> /dev/null; then
        log ERROR "Docker daemon is not running"
        return 2
    fi

    log SUCCESS "Docker is running"
    return 0
}

# Check Docker Compose services
check_docker_compose() {
    log INFO "Checking Docker Compose services..."

    cd "$PROJECT_DIR"

    if ! docker compose ps &> /dev/null; then
        log ERROR "Docker Compose is not available"
        return 2
    fi

    local services=$(docker compose ps --services)
    local running_count=0
    local total_count=0

    for service in $services; do
        total_count=$((total_count + 1))
        local status=$(docker compose ps "$service" --format json 2>/dev/null | grep -o '"State":"[^"]*"' | cut -d'"' -f4)

        if [ "$status" = "running" ]; then
            running_count=$((running_count + 1))
            HEALTHY_SERVICES+=("$service")
            if [ "$VERBOSE" = true ]; then
                log SUCCESS "Service $service is running"
            fi
        else
            FAILED_SERVICES+=("$service:$status")
            log ERROR "Service $service is $status"
        fi
    done

    log INFO "Services: $running_count/$total_count running"

    if [ $running_count -eq $total_count ]; then
        return 0
    elif [ $running_count -eq 0 ]; then
        return 2
    else
        return 1
    fi
}

# Check PostgreSQL
check_postgres() {
    log INFO "Checking PostgreSQL..."

    local container_name=$(docker compose ps postgres --format json 2>/dev/null | grep -o '"Name":"[^"]*"' | cut -d'"' -f4)

    if [ -z "$container_name" ]; then
        log ERROR "PostgreSQL container not found"
        FAILED_SERVICES+=("postgres:not_found")
        return 1
    fi

    local db_check=$(docker exec "$container_name" pg_isready -U postgres 2>&1)

    if echo "$db_check" | grep -q "accepting connections"; then
        log SUCCESS "PostgreSQL is accepting connections"

        # Check database size
        if [ "$VERBOSE" = true ]; then
            local db_size=$(docker exec "$container_name" psql -U postgres -t -c "SELECT pg_size_pretty(pg_database_size('scrdesk'));" 2>/dev/null | xargs)
            log INFO "Database size: $db_size"
        fi

        return 0
    else
        log ERROR "PostgreSQL is not accepting connections"
        FAILED_SERVICES+=("postgres:connection_failed")
        return 1
    fi
}

# Check Redis
check_redis() {
    log INFO "Checking Redis..."

    local container_name=$(docker compose ps redis --format json 2>/dev/null | grep -o '"Name":"[^"]*"' | cut -d'"' -f4)

    if [ -z "$container_name" ]; then
        log ERROR "Redis container not found"
        FAILED_SERVICES+=("redis:not_found")
        return 1
    fi

    local redis_check=$(docker exec "$container_name" redis-cli ping 2>&1)

    if [ "$redis_check" = "PONG" ]; then
        log SUCCESS "Redis is responding"

        # Check memory usage
        if [ "$VERBOSE" = true ]; then
            local memory=$(docker exec "$container_name" redis-cli info memory 2>/dev/null | grep "used_memory_human" | cut -d':' -f2 | tr -d '\r')
            log INFO "Redis memory: $memory"
        fi

        return 0
    else
        log ERROR "Redis is not responding"
        FAILED_SERVICES+=("redis:connection_failed")
        return 1
    fi
}

# Check API endpoints
check_api_endpoints() {
    log INFO "Checking API endpoints..."

    local services=(
        "auth-service:8081"
        "device-manager:8082"
        "policy-engine:8083"
        "core-server:8084"
        "relay-cluster:8085"
        "audit-service:8086"
        "notification-service:8087"
        "billing-service:8088"
        "admin-backend:8089"
        "update-server:8090"
        "analytics:8091"
    )

    local healthy_count=0
    local total_count=${#services[@]}

    for service_port in "${services[@]}"; do
        IFS=':' read -r service port <<< "$service_port"

        # Try to connect to health endpoint
        local response=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout 5 "http://localhost:$port/health" 2>/dev/null || echo "000")

        if [ "$response" = "200" ]; then
            healthy_count=$((healthy_count + 1))
            if [ "$VERBOSE" = true ]; then
                log SUCCESS "API $service health check passed"
            fi
        else
            log ERROR "API $service health check failed (HTTP $response)"
            FAILED_SERVICES+=("$service:http_$response")
        fi
    done

    log INFO "API endpoints: $healthy_count/$total_count healthy"

    if [ $healthy_count -eq $total_count ]; then
        return 0
    elif [ $healthy_count -eq 0 ]; then
        return 2
    else
        return 1
    fi
}

# Check disk space
check_disk_space() {
    log INFO "Checking disk space..."

    local threshold=90
    local usage=$(df -h / | awk 'NR==2 {print $5}' | sed 's/%//')

    if [ "$usage" -lt "$threshold" ]; then
        log SUCCESS "Disk space: ${usage}% used"
        return 0
    else
        log WARNING "Disk space critical: ${usage}% used (threshold: ${threshold}%)"
        return 1
    fi
}

# Check memory usage
check_memory() {
    log INFO "Checking memory usage..."

    if command -v free &> /dev/null; then
        local mem_usage=$(free | awk 'NR==2 {printf "%.0f", $3/$2 * 100}')

        if [ "$mem_usage" -lt 90 ]; then
            log SUCCESS "Memory: ${mem_usage}% used"
            return 0
        else
            log WARNING "Memory usage high: ${mem_usage}%"
            return 1
        fi
    else
        log WARNING "Cannot check memory (free command not available)"
        return 0
    fi
}

# Check container logs for errors
check_logs() {
    if [ "$VERBOSE" != true ]; then
        return 0
    fi

    log INFO "Checking container logs for errors..."

    local services=$(docker compose ps --services 2>/dev/null)
    local error_count=0

    for service in $services; do
        local errors=$(docker compose logs --tail=100 "$service" 2>/dev/null | grep -i "error\|panic\|fatal" | wc -l)

        if [ "$errors" -gt 0 ]; then
            log WARNING "Service $service has $errors error lines in recent logs"
            error_count=$((error_count + errors))
        fi
    done

    if [ $error_count -gt 0 ]; then
        log WARNING "Total error lines found: $error_count"
    else
        log SUCCESS "No errors found in recent logs"
    fi
}

# Send alert notification
send_alert() {
    local subject="$1"
    local message="$2"

    if [ "$SEND_ALERTS" != true ]; then
        return 0
    fi

    # Send email if configured
    if [ -n "$ALERT_EMAIL" ] && command -v mail &> /dev/null; then
        echo "$message" | mail -s "$subject" "$ALERT_EMAIL"
        log INFO "Alert email sent to $ALERT_EMAIL"
    fi

    # Send webhook if configured
    if [ -n "$ALERT_WEBHOOK" ]; then
        curl -X POST "$ALERT_WEBHOOK" \
            -H "Content-Type: application/json" \
            -d "{\"subject\": \"$subject\", \"message\": \"$message\"}" \
            &> /dev/null
        log INFO "Alert webhook sent"
    fi
}

# Generate JSON report
generate_json_report() {
    local status=$1
    local status_text=""

    case $status in
        0) status_text="healthy" ;;
        1) status_text="degraded" ;;
        2) status_text="critical" ;;
    esac

    cat << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "status": "$status_text",
  "exit_code": $status,
  "healthy_services": $(printf '%s\n' "${HEALTHY_SERVICES[@]}" | jq -R . | jq -s .),
  "failed_services": $(printf '%s\n' "${FAILED_SERVICES[@]}" | jq -R . | jq -s .),
  "checks": {
    "docker": $([ $DOCKER_STATUS -eq 0 ] && echo "true" || echo "false"),
    "postgres": $([ $POSTGRES_STATUS -eq 0 ] && echo "true" || echo "false"),
    "redis": $([ $REDIS_STATUS -eq 0 ] && echo "true" || echo "false"),
    "services": $([ $SERVICES_STATUS -eq 0 ] && echo "true" || echo "false"),
    "api": $([ $API_STATUS -eq 0 ] && echo "true" || echo "false")
  }
}
EOF
}

# Main execution
main() {
    if [ "$JSON_OUTPUT" != "true" ]; then
        log INFO "Starting ScrDesk health check..."
        log INFO "Timestamp: $(date)"
        echo ""
    fi

    # Run all checks
    check_docker
    DOCKER_STATUS=$?

    if [ $DOCKER_STATUS -ne 0 ]; then
        log ERROR "Critical: Docker is not available"
        if [ "$JSON_OUTPUT" = "true" ]; then
            echo '{"status":"critical","error":"docker_unavailable"}'
        fi
        exit 2
    fi

    check_docker_compose
    SERVICES_STATUS=$?

    check_postgres
    POSTGRES_STATUS=$?

    check_redis
    REDIS_STATUS=$?

    check_api_endpoints
    API_STATUS=$?

    check_disk_space
    DISK_STATUS=$?

    check_memory
    MEMORY_STATUS=$?

    check_logs

    # Determine overall status
    local exit_code=0

    if [ $DOCKER_STATUS -eq 2 ] || [ $SERVICES_STATUS -eq 2 ] || [ $POSTGRES_STATUS -eq 2 ] || [ $REDIS_STATUS -eq 2 ]; then
        exit_code=2
    elif [ ${#FAILED_SERVICES[@]} -gt 0 ]; then
        exit_code=1
    fi

    # Generate output
    if [ "$JSON_OUTPUT" = "true" ]; then
        generate_json_report $exit_code
    else
        echo ""
        log INFO "==================== Summary ===================="
        log INFO "Healthy services: ${#HEALTHY_SERVICES[@]}"
        log INFO "Failed services: ${#FAILED_SERVICES[@]}"

        if [ ${#FAILED_SERVICES[@]} -gt 0 ]; then
            echo ""
            log ERROR "Failed services:"
            for service in "${FAILED_SERVICES[@]}"; do
                log ERROR "  - $service"
            done
        fi

        echo ""
        case $exit_code in
            0)
                log SUCCESS "All systems operational"
                ;;
            1)
                log WARNING "System degraded - some services are failing"
                ;;
            2)
                log ERROR "System critical - major infrastructure failure"
                ;;
        esac
        log INFO "================================================="
    fi

    # Send alerts if necessary
    if [ $exit_code -ne 0 ]; then
        local alert_subject="ScrDesk Health Check: "
        case $exit_code in
            1) alert_subject+="DEGRADED" ;;
            2) alert_subject+="CRITICAL" ;;
        esac

        local alert_message="Health check failed at $(date)\n\nFailed services:\n"
        for service in "${FAILED_SERVICES[@]}"; do
            alert_message+="- $service\n"
        done

        send_alert "$alert_subject" "$alert_message"
    fi

    exit $exit_code
}

# Create log directory if needed
if [ "$JSON_OUTPUT" != "true" ] && [ -w "/var/log" ]; then
    mkdir -p "$(dirname "$LOG_FILE")" 2>/dev/null || true
fi

# Run main function
main
