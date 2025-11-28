export default function Home() {
  return <div className="min-h-screen flex items-center justify-center bg-gray-100">
    <div className="text-center">
      <h1 className="text-4xl font-bold mb-4">ScrDesk PRO Enterprise</h1>
      <p className="text-gray-600 mb-8">Admin Panel</p>
      <a href="/login" className="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700">
        Login
      </a>
    </div>
  </div>
}
