import { useState } from 'react'
import { Archive, Search, Network, Users } from 'lucide-react'

function App() {
  const [url, setUrl] = useState('')
  const [searchQuery, setSearchQuery] = useState('')

  const handleArchive = async () => {
    if (!url) return
    console.log('Archiving:', url)
    // TODO: Call EternalWeb API
    alert(`Archiving: ${url}`)
  }

  const handleSearch = async () => {
    if (!searchQuery) return
    console.log('Searching:', searchQuery)
    // TODO: Call EternalWeb API
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
      {/* Header */}
      <header className="bg-white shadow-md">
        <div className="container mx-auto px-4 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <Archive className="w-8 h-8 text-eternal-primary" />
              <h1 className="text-3xl font-bold text-eternal-dark">EternalWeb</h1>
            </div>
            <nav className="flex space-x-6">
              <a href="#" className="text-gray-700 hover:text-eternal-primary">Archive</a>
              <a href="#" className="text-gray-700 hover:text-eternal-primary">Search</a>
              <a href="#" className="text-gray-700 hover:text-eternal-primary">Collectives</a>
              <button className="bg-eternal-primary text-white px-4 py-2 rounded-lg hover:bg-blue-600">
                Connect Wallet
              </button>
            </nav>
          </div>
        </div>
      </header>

      {/* Hero Section */}
      <main className="container mx-auto px-4 py-12">
        <div className="text-center mb-12">
          <h2 className="text-5xl font-bold text-eternal-dark mb-4">
            Reclaiming the Internet for Humankind
          </h2>
          <p className="text-xl text-gray-600 max-w-3xl mx-auto">
            Decentralized web archiving platform built for Web 3.0 and Web 4.0.
            Preserve, share, and discover content without corporate gatekeepers.
          </p>
        </div>

        {/* Archive Section */}
        <div className="bg-white rounded-2xl shadow-xl p-8 mb-8">
          <div className="flex items-center mb-6">
            <Archive className="w-6 h-6 text-eternal-primary mr-3" />
            <h3 className="text-2xl font-bold">Archive a Page</h3>
          </div>
          <div className="flex space-x-4">
            <input
              type="url"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://example.com"
              className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-eternal-primary"
            />
            <button
              onClick={handleArchive}
              className="bg-eternal-primary text-white px-8 py-3 rounded-lg hover:bg-blue-600 font-semibold"
            >
              Archive
            </button>
          </div>
        </div>

        {/* Search Section */}
        <div className="bg-white rounded-2xl shadow-xl p-8 mb-8">
          <div className="flex items-center mb-6">
            <Search className="w-6 h-6 text-eternal-secondary mr-3" />
            <h3 className="text-2xl font-bold">Search Archives</h3>
          </div>
          <div className="flex space-x-4">
            <input
              type="text"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              placeholder="Search decentralized archives..."
              className="flex-1 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-eternal-secondary"
            />
            <button
              onClick={handleSearch}
              className="bg-eternal-secondary text-white px-8 py-3 rounded-lg hover:bg-green-600 font-semibold"
            >
              Search
            </button>
          </div>
        </div>

        {/* Features Grid */}
        <div className="grid md:grid-cols-3 gap-8">
          <div className="bg-white rounded-xl shadow-lg p-6">
            <Network className="w-12 h-12 text-eternal-primary mb-4" />
            <h4 className="text-xl font-bold mb-2">Decentralized</h4>
            <p className="text-gray-600">
              P2P network with no single point of failure. Your data, your control.
            </p>
          </div>
          <div className="bg-white rounded-xl shadow-lg p-6">
            <Archive className="w-12 h-12 text-eternal-secondary mb-4" />
            <h4 className="text-xl font-bold mb-2">Permanent Storage</h4>
            <p className="text-gray-600">
              IPFS and Hypercore ensure your archives live forever, immune to link rot.
            </p>
          </div>
          <div className="bg-white rounded-xl shadow-lg p-6">
            <Users className="w-12 h-12 text-indigo-600 mb-4" />
            <h4 className="text-xl font-bold mb-2">Community Governed</h4>
            <p className="text-gray-600">
              DAO-based governance. The community decides the future of EternalWeb.
            </p>
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="bg-eternal-dark text-white mt-16 py-8">
        <div className="container mx-auto px-4 text-center">
          <p>EternalWeb v0.1.0 - Built with ❤️ by the community for humankind</p>
          <p className="mt-2 text-sm text-gray-400">
            Licensed under AGPL-3.0 | <a href="https://github.com/e_honiball/eternalweb" className="underline">GitHub</a>
          </p>
        </div>
      </footer>
    </div>
  )
}

export default App
