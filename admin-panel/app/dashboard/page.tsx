export default function Dashboard() {
  return <div className="min-h-screen bg-gray-100 p-8">
    <h1 className="text-3xl font-bold mb-6">Dashboard</h1>
    <div className="grid grid-cols-4 gap-4">
      <div className="bg-white p-6 rounded-lg shadow"><h3 className="text-gray-600">Total Tenants</h3><p className="text-3xl font-bold">42</p></div>
      <div className="bg-white p-6 rounded-lg shadow"><h3 className="text-gray-600">Total Users</h3><p className="text-3xl font-bold">1,234</p></div>
      <div className="bg-white p-6 rounded-lg shadow"><h3 className="text-gray-600">Active Devices</h3><p className="text-3xl font-bold">567</p></div>
      <div className="bg-white p-6 rounded-lg shadow"><h3 className="text-gray-600">Active Sessions</h3><p className="text-3xl font-bold">89</p></div>
    </div>
  </div>
}
