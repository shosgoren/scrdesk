#!/bin/bash

##############################################################################
# ScrDesk PRO Enterprise - Real-time Monitoring Dashboard
#
# This script displays a real-time dashboard of all services with metrics
# including CPU, memory, network I/O, and container status.
#
# Usage:
#   ./monitor-dashboard.sh
#
# Requirements:
#   - Docker
#   - Docker Compose
#   - jq (for JSON parsing)
##############################################################################

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Configuration
REFRESH_INTERVAL=2
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

# Function to clear screen and move cursor to top
clear_screen() {
    clear
}

# Function to format bytes
format_bytes() {
    local bytes=$1
    if [ $bytes -lt 1024 ]; then
        echo "${bytes}B"
    elif [ $bytes -lt 1048576 ]; then
        echo "$(awk "BEGIN {printf \"%.1f\", $bytes/1024}")K"
    elif [ $bytes -lt 1073741824 ]; then
        echo "$(awk "BEGIN {printf \"%.1f\", $bytes/1048576}")M"
    else
        echo "$(awk "BEGIN {printf \"%.2f\", $bytes/1073741824}")G"
    fi
}

# Function to get container stats
get_container_stats() {
    local container=$1

    # Get stats in JSON format
    local stats=$(docker stats "$container" --no-stream --format "{{json .}}" 2>/dev/null)

    if [ -z "$stats" ]; then
        echo "N/A|N/A|N/A|N/A"
        return
    fi

    local cpu=$(echo "$stats" | jq -r '.CPUPerc' | sed 's/%//')
    local mem=$(echo "$stats" | jq -r '.MemUsage' | cut -d'/' -f1)
    local net=$(echo "$stats" | jq -r '.NetIO')
    local block=$(echo "$stats" | jq -r '.BlockIO')

    echo "$cpu|$mem|$net|$block"
}

# Function to get container uptime
get_container_uptime() {
    local container=$1
    docker inspect "$container" --format='{{.State.StartedAt}}' 2>/dev/null | xargs -I{} date -d {} '+%s' 2>/dev/null || echo "0"
}

# Function to print header
print_header() {
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')

    echo -e "${BOLD}${CYAN}╔════════════════════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BOLD}${CYAN}║${NC}  ${BOLD}ScrDesk PRO Enterprise - Real-time Monitoring Dashboard${NC}                    ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}║${NC}  ${timestamp}                                                            ${BOLD}${CYAN}║${NC}"
    echo -e "${BOLD}${CYAN}╚════════════════════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
}

# Function to print service row
print_service_row() {
    local service=$1
    local status=$2
    local cpu=$3
    local mem=$4
    local uptime=$5

    # Color based on status
    local status_color=$GREEN
    local status_icon="●"

    case $status in
        running)
            status_color=$GREEN
            status_icon="●"
            ;;
        exited)
            status_color=$RED
            status_icon="●"
            ;;
        restarting)
            status_color=$YELLOW
            status_icon="⟳"
            ;;
        *)
            status_color=$RED
            status_icon="✗"
            ;;
    esac

    # CPU color
    local cpu_color=$GREEN
    if (( $(echo "$cpu > 80" | bc -l 2>/dev/null || echo 0) )); then
        cpu_color=$RED
    elif (( $(echo "$cpu > 50" | bc -l 2>/dev/null || echo 0) )); then
        cpu_color=$YELLOW
    fi

    # Format uptime
    local uptime_str="N/A"
    if [ "$uptime" != "0" ] && [ "$uptime" != "N/A" ]; then
        local current=$(date +%s)
        local diff=$((current - uptime))
        local days=$((diff / 86400))
        local hours=$(((diff % 86400) / 3600))
        local minutes=$(((diff % 3600) / 60))

        if [ $days -gt 0 ]; then
            uptime_str="${days}d ${hours}h"
        elif [ $hours -gt 0 ]; then
            uptime_str="${hours}h ${minutes}m"
        else
            uptime_str="${minutes}m"
        fi
    fi

    # Format service name (truncate if too long)
    local service_display=$(printf "%-25s" "${service:0:25}")

    # Format CPU and memory
    local cpu_display=$(printf "%5s%%" "$cpu")
    local mem_display=$(printf "%10s" "$mem")
    local uptime_display=$(printf "%10s" "$uptime_str")

    echo -e "  ${status_color}${status_icon}${NC}  ${service_display}  ${cpu_color}${cpu_display}${NC}  ${mem_display}  ${uptime_display}"
}

# Function to check API health
check_api_health() {
    local port=$1
    local response=$(curl -s -o /dev/null -w "%{http_code}" --connect-timeout 1 "http://localhost:$port/health" 2>/dev/null || echo "000")

    if [ "$response" = "200" ]; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${RED}✗${NC}"
    fi
}

# Function to print infrastructure status
print_infrastructure() {
    echo -e "${BOLD}${BLUE}Infrastructure Status:${NC}"
    echo -e "  ────────────────────────────────────────────────────────────────"

    # PostgreSQL
    local pg_container=$(docker compose -f "$PROJECT_DIR/docker-compose.yml" ps -q postgres 2>/dev/null)
    if [ -n "$pg_container" ]; then
        local pg_status=$(docker inspect "$pg_container" --format='{{.State.Status}}' 2>/dev/null)
        local pg_health=$(docker exec "$pg_container" pg_isready -U postgres 2>&1 | grep -q "accepting connections" && echo "✓" || echo "✗")

        local stats=$(get_container_stats "$pg_container")
        IFS='|' read -r cpu mem net block <<< "$stats"

        if [ "$pg_status" = "running" ] && [ "$pg_health" = "✓" ]; then
            echo -e "  ${GREEN}●${NC} PostgreSQL         ${GREEN}✓${NC} Online    CPU: $cpu%  Mem: $mem"
        else
            echo -e "  ${RED}●${NC} PostgreSQL         ${RED}✗${NC} Offline"
        fi
    else
        echo -e "  ${RED}●${NC} PostgreSQL         ${RED}✗${NC} Not Found"
    fi

    # Redis
    local redis_container=$(docker compose -f "$PROJECT_DIR/docker-compose.yml" ps -q redis 2>/dev/null)
    if [ -n "$redis_container" ]; then
        local redis_status=$(docker inspect "$redis_container" --format='{{.State.Status}}' 2>/dev/null)
        local redis_health=$(docker exec "$redis_container" redis-cli ping 2>&1 | grep -q "PONG" && echo "✓" || echo "✗")

        local stats=$(get_container_stats "$redis_container")
        IFS='|' read -r cpu mem net block <<< "$stats"

        if [ "$redis_status" = "running" ] && [ "$redis_health" = "✓" ]; then
            echo -e "  ${GREEN}●${NC} Redis              ${GREEN}✓${NC} Online    CPU: $cpu%  Mem: $mem"
        else
            echo -e "  ${RED}●${NC} Redis              ${RED}✗${NC} Offline"
        fi
    else
        echo -e "  ${RED}●${NC} Redis              ${RED}✗${NC} Not Found"
    fi

    echo ""
}

# Function to print microservices status
print_microservices() {
    echo -e "${BOLD}${BLUE}Microservices:${NC}"
    echo -e "  ────────────────────────────────────────────────────────────────────────────"
    printf "     %-27s %7s  %10s  %10s  %6s\n" "SERVICE" "CPU" "MEMORY" "UPTIME" "HEALTH"
    echo -e "  ────────────────────────────────────────────────────────────────────────────"

    local services=(
        "scrdesk-auth-service:8081"
        "scrdesk-device-manager:8082"
        "scrdesk-policy-engine:8083"
        "scrdesk-core-server:8084"
        "scrdesk-relay-cluster:8085"
        "scrdesk-audit-service:8086"
        "scrdesk-notification-service:8087"
        "scrdesk-billing-service:8088"
        "scrdesk-admin-backend:8089"
        "scrdesk-update-server:8090"
        "scrdesk-analytics:8091"
    )

    local total=0
    local running=0

    for service_port in "${services[@]}"; do
        IFS=':' read -r service port <<< "$service_port"
        total=$((total + 1))

        local container=$(docker compose -f "$PROJECT_DIR/docker-compose.yml" ps -q "$service" 2>/dev/null)

        if [ -n "$container" ]; then
            local status=$(docker inspect "$container" --format='{{.State.Status}}' 2>/dev/null)
            local uptime=$(get_container_uptime "$container")

            local stats=$(get_container_stats "$container")
            IFS='|' read -r cpu mem net block <<< "$stats"

            if [ "$cpu" = "N/A" ]; then
                cpu="0.0"
            fi
            if [ "$mem" = "N/A" ]; then
                mem="0B"
            fi

            local health=$(check_api_health "$port")

            if [ "$status" = "running" ]; then
                running=$((running + 1))
            fi

            print_service_row "$service" "$status" "$cpu" "$mem" "$uptime"
            echo -n "                                                                      $health"
            echo ""
        else
            print_service_row "$service" "not_found" "0.0" "0B" "N/A"
            echo -e "                                                                      ${RED}✗${NC}"
        fi
    done

    echo ""
    echo -e "  ${BOLD}Summary:${NC} $running/$total services running"
    echo ""
}

# Function to print system resources
print_system_resources() {
    echo -e "${BOLD}${BLUE}System Resources:${NC}"
    echo -e "  ────────────────────────────────────────────────────────────────"

    # Disk usage
    local disk_usage=$(df -h / | awk 'NR==2 {print $5}')
    local disk_avail=$(df -h / | awk 'NR==2 {print $4}')

    local disk_color=$GREEN
    local disk_pct=$(echo "$disk_usage" | sed 's/%//')
    if [ "$disk_pct" -gt 80 ]; then
        disk_color=$RED
    elif [ "$disk_pct" -gt 60 ]; then
        disk_color=$YELLOW
    fi

    echo -e "  Disk Usage:     ${disk_color}${disk_usage}${NC} used (${disk_avail} available)"

    # Memory usage
    if command -v free &> /dev/null; then
        local mem_total=$(free -h | awk 'NR==2 {print $2}')
        local mem_used=$(free -h | awk 'NR==2 {print $3}')
        local mem_pct=$(free | awk 'NR==2 {printf "%.0f", $3/$2 * 100}')

        local mem_color=$GREEN
        if [ "$mem_pct" -gt 80 ]; then
            mem_color=$RED
        elif [ "$mem_pct" -gt 60 ]; then
            mem_color=$YELLOW
        fi

        echo -e "  Memory Usage:   ${mem_color}${mem_pct}%${NC} (${mem_used}/${mem_total})"
    fi

    # Docker stats
    local container_count=$(docker ps -q | wc -l)
    local image_count=$(docker images -q | wc -l)

    echo -e "  Containers:     ${container_count} running"
    echo -e "  Images:         ${image_count} total"

    echo ""
}

# Function to print recent logs
print_recent_logs() {
    echo -e "${BOLD}${BLUE}Recent Activity (Last 5 log entries):${NC}"
    echo -e "  ────────────────────────────────────────────────────────────────────────────"

    local services=$(docker compose -f "$PROJECT_DIR/docker-compose.yml" ps --services 2>/dev/null | head -5)

    for service in $services; do
        local container=$(docker compose -f "$PROJECT_DIR/docker-compose.yml" ps -q "$service" 2>/dev/null)
        if [ -n "$container" ]; then
            echo -e "  ${CYAN}[$service]${NC}"
            docker logs "$container" --tail=1 2>&1 | sed 's/^/    /' | head -1
        fi
    done

    echo ""
}

# Main loop
main() {
    cd "$PROJECT_DIR" || exit 1

    # Check if docker-compose.yml exists
    if [ ! -f "docker-compose.yml" ]; then
        echo "Error: docker-compose.yml not found in $PROJECT_DIR"
        exit 1
    fi

    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        echo "Error: Docker is not installed"
        exit 1
    fi

    # Check if jq is available (optional but recommended)
    if ! command -v jq &> /dev/null; then
        echo "Warning: jq is not installed. Some features may not work correctly."
        echo "Install with: apt-get install jq (Debian/Ubuntu) or brew install jq (macOS)"
        sleep 2
    fi

    # Trap Ctrl+C
    trap 'echo ""; echo "Monitoring stopped."; exit 0' INT

    echo "Starting monitoring dashboard..."
    echo "Press Ctrl+C to exit"
    sleep 2

    while true; do
        clear_screen
        print_header
        print_infrastructure
        print_microservices
        print_system_resources
        # print_recent_logs  # Uncomment if you want to see logs

        echo -e "${CYAN}Refreshing every ${REFRESH_INTERVAL}s... (Press Ctrl+C to exit)${NC}"

        sleep $REFRESH_INTERVAL
    done
}

# Run main function
main
