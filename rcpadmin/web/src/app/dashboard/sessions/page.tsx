"use client"

import { useState, useEffect } from "react"
import { BarChart, Activity, RefreshCcw, XCircle, Info } from "lucide-react"
import { getSessions, closeSession } from "@/lib/api"
import { useToast } from "@/components/ui/use-toast"
import { format } from "date-fns"

interface Session {
  id: string
  client_name: string
  client_address: string
  client_version?: string
  authenticated: boolean
  connected_at: string
  last_activity?: string
  application?: string
}

export default function SessionsPage() {
  const [loading, setLoading] = useState(true)
  const [sessions, setSessions] = useState<Session[]>([])
  const [selectedSession, setSelectedSession] = useState<Session | null>(null)
  const { toast } = useToast()

  useEffect(() => {
    fetchSessions()
  }, [])

  async function fetchSessions() {
    setLoading(true)
    try {
      const response = await getSessions()
      
      if (response.error) {
        toast({
          title: "Error",
          description: response.error.message,
          variant: "destructive",
        })
        return
      }
      
      setSessions(response.data)
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to fetch session data",
        variant: "destructive",
      })
    } finally {
      setLoading(false)
    }
  }

  const handleCloseSession = async (id: string) => {
    if (!confirm("Are you sure you want to terminate this client session?")) {
      return
    }

    try {
      const response = await closeSession(id)
      
      if (response.error) {
        toast({
          title: "Error",
          description: response.error.message,
          variant: "destructive",
        })
        return
      }
      
      toast({
        title: "Success",
        description: "Session terminated successfully",
      })
      
      // If we were viewing details of the closed session, clear it
      if (selectedSession?.id === id) {
        setSelectedSession(null)
      }
      
      // Refresh the sessions list
      fetchSessions()
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to terminate session",
        variant: "destructive",
      })
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Active Sessions</h2>
          <p className="text-gray-500 dark:text-gray-400">
            Monitor and manage client connections
          </p>
        </div>
        <button
          onClick={fetchSessions}
          className="inline-flex items-center rounded-md bg-white px-3 py-2 text-sm font-medium text-gray-700 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 dark:bg-gray-800 dark:text-white dark:ring-gray-600 dark:hover:bg-gray-700"
        >
          <RefreshCcw className="mr-2 h-4 w-4" />
          Refresh
        </button>
      </div>

      {loading ? (
        <div className="py-8 text-center">
          <div className="mx-auto h-8 w-8 animate-spin rounded-full border-4 border-b-transparent"></div>
          <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">Loading sessions...</p>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
          <div className="lg:col-span-2">
            <div className="rounded-lg border bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
              <div className="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">Connected Clients</h3>
              </div>
              
              <div className="overflow-hidden">
                {sessions.length === 0 ? (
                  <div className="py-8 text-center">
                    <p className="text-gray-500 dark:text-gray-400">No active sessions</p>
                  </div>
                ) : (
                  <table className="w-full text-left">
                    <thead className="bg-gray-50 text-xs uppercase text-gray-500 dark:bg-gray-700 dark:text-gray-400">
                      <tr>
                        <th className="px-6 py-3">Client</th>
                        <th className="px-6 py-3">Address</th>
                        <th className="px-6 py-3">Status</th>
                        <th className="px-6 py-3">Connected At</th>
                        <th className="px-6 py-3">Actions</th>
                      </tr>
                    </thead>
                    <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
                      {sessions.map((session) => (
                        <tr 
                          key={session.id} 
                          className={`bg-white hover:bg-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700 ${
                            selectedSession?.id === session.id ? 'bg-blue-50 dark:bg-blue-900/20' : ''
                          }`}
                        >
                          <td 
                            className="cursor-pointer whitespace-nowrap px-6 py-4 font-medium text-gray-900 dark:text-white"
                            onClick={() => setSelectedSession(session)}
                          >
                            {session.client_name || 'Unknown Client'}
                            {session.client_version && (
                              <p className="text-xs text-gray-500 dark:text-gray-400">
                                v{session.client_version}
                              </p>
                            )}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4 text-gray-500 dark:text-gray-400">
                            {session.client_address}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4">
                            <span className={`rounded-full px-2.5 py-0.5 text-xs font-medium ${
                              session.authenticated 
                                ? "bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-300"
                                : "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300"
                            }`}>
                              {session.authenticated ? "Authenticated" : "Connected"}
                            </span>
                          </td>
                          <td className="whitespace-nowrap px-6 py-4 text-gray-500 dark:text-gray-400">
                            {session.connected_at ? format(new Date(session.connected_at), 'MMM d, yyyy h:mm a') : 'N/A'}
                          </td>
                          <td className="whitespace-nowrap px-6 py-4">
                            <div className="flex items-center space-x-2">
                              <button
                                onClick={() => setSelectedSession(session)}
                                className="rounded-md bg-gray-100 p-2 text-gray-700 hover:bg-gray-200 dark:bg-gray-700 dark:text-gray-300 dark:hover:bg-gray-600"
                                title="View details"
                              >
                                <Info className="h-4 w-4" />
                              </button>
                              <button
                                onClick={() => handleCloseSession(session.id)}
                                className="rounded-md bg-red-100 p-2 text-red-700 hover:bg-red-200 dark:bg-red-900/30 dark:text-red-300 dark:hover:bg-red-900/50"
                                title="Terminate session"
                              >
                                <XCircle className="h-4 w-4" />
                              </button>
                            </div>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                )}
              </div>
              
              <div className="border-t border-gray-200 px-6 py-3 dark:border-gray-700">
                <div className="flex items-center justify-between">
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {sessions.length} {sessions.length === 1 ? 'client' : 'clients'} connected
                  </p>
                  <div className="flex items-center space-x-2">
                    <Activity className="h-4 w-4 text-gray-500 dark:text-gray-400" />
                    <span className="text-sm text-gray-500 dark:text-gray-400">
                      Real-time monitoring
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          {/* Session details panel */}
          <div className="col-span-1">
            <div className="sticky top-4 rounded-lg border bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
              <div className="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">Session Details</h3>
              </div>
              
              <div className="p-6">
                {selectedSession ? (
                  <div className="space-y-4">
                    <div>
                      <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Client ID</h4>
                      <p className="mt-1 text-sm font-mono text-gray-900 dark:text-white">
                        {selectedSession.id}
                      </p>
                    </div>
                    
                    <div>
                      <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Client Name</h4>
                      <p className="mt-1 text-sm text-gray-900 dark:text-white">
                        {selectedSession.client_name || 'Unknown'}
                      </p>
                    </div>
                    
                    <div>
                      <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Version</h4>
                      <p className="mt-1 text-sm text-gray-900 dark:text-white">
                        {selectedSession.client_version || 'Unknown'}
                      </p>
                    </div>
                    
                    <div>
                      <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">IP Address</h4>
                      <p className="mt-1 text-sm font-mono text-gray-900 dark:text-white">
                        {selectedSession.client_address}
                      </p>
                    </div>
                    
                    <div>
                      <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Connected Since</h4>
                      <p className="mt-1 text-sm text-gray-900 dark:text-white">
                        {selectedSession.connected_at ? format(new Date(selectedSession.connected_at), 'PPpp') : 'Unknown'}
                      </p>
                    </div>
                    
                    {selectedSession.last_activity && (
                      <div>
                        <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Last Activity</h4>
                        <p className="mt-1 text-sm text-gray-900 dark:text-white">
                          {format(new Date(selectedSession.last_activity), 'PPpp')}
                        </p>
                      </div>
                    )}
                    
                    {selectedSession.application && (
                      <div>
                        <h4 className="text-sm font-medium text-gray-500 dark:text-gray-400">Running Application</h4>
                        <p className="mt-1 text-sm text-gray-900 dark:text-white">
                          {selectedSession.application}
                        </p>
                      </div>
                    )}
                    
                    <div className="pt-4">
                      <button
                        onClick={() => handleCloseSession(selectedSession.id)}
                        className="inline-flex w-full items-center justify-center rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-2"
                      >
                        <XCircle className="mr-2 h-4 w-4" />
                        Terminate Session
                      </button>
                    </div>
                  </div>
                ) : (
                  <div className="flex flex-col items-center justify-center py-8">
                    <BarChart className="h-12 w-12 text-gray-300 dark:text-gray-600" />
                    <p className="mt-2 text-center text-sm text-gray-500 dark:text-gray-400">
                      Select a client to view session details
                    </p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}