"use client"

import { useEffect, useState } from "react"
import { Activity, MonitorPlay, Users, Server, Clock } from "lucide-react"
import { getServerStatus, getSessions, getApplications, getSystemMetrics } from "@/lib/api"
import { format } from "date-fns"
import { 
  LineChart, 
  Line, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer,
  AreaChart,
  Area
} from "recharts"

// Mock data for charts
const generateMockData = () => {
  const now = new Date()
  return Array.from({ length: 24 }, (_, i) => {
    const time = new Date(now)
    time.setHours(now.getHours() - 23 + i)
    return {
      time: format(time, 'HH:mm'),
      cpu: Math.floor(Math.random() * 50) + 20,
      memory: Math.floor(Math.random() * 40) + 30,
      connections: Math.floor(Math.random() * 15),
    }
  })
}

export default function DashboardPage() {
  const [loading, setLoading] = useState(true)
  const [serverStatus, setServerStatus] = useState<any>(null)
  const [sessions, setSessions] = useState<any[]>([])
  const [applications, setApplications] = useState<any[]>([])
  const [metrics, setMetrics] = useState<any>(null)
  const [chartData] = useState(() => generateMockData())

  useEffect(() => {
    async function fetchDashboardData() {
      setLoading(true)
      try {
        // Fetch server status
        const statusRes = await getServerStatus()
        if (statusRes.data) {
          setServerStatus(statusRes.data)
        }

        // Fetch active sessions
        const sessionsRes = await getSessions()
        if (sessionsRes.data) {
          setSessions(sessionsRes.data)
        }

        // Fetch registered applications
        const appsRes = await getApplications()
        if (appsRes.data) {
          setApplications(appsRes.data)
        }

        // Fetch system metrics
        const metricsRes = await getSystemMetrics()
        if (metricsRes.data) {
          setMetrics(metricsRes.data)
        }
      } catch (error) {
        console.error("Error fetching dashboard data", error)
      } finally {
        setLoading(false)
      }
    }

    fetchDashboardData()
    
    // Refresh data every 30 seconds
    const intervalId = setInterval(fetchDashboardData, 30000)
    return () => clearInterval(intervalId)
  }, [])

  // When data is loading, show a loading state
  if (loading && !serverStatus) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-b-transparent"></div>
          <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">Loading dashboard...</p>
        </div>
      </div>
    )
  }

  // Calculate values for statistics cards (use mock data if API fails)
  const activeSessions = sessions.length || 0
  const registeredApps = applications.length || 0
  const cpuUsage = metrics?.cpu_usage || Math.floor(Math.random() * 50) + 20
  const memoryUsage = metrics?.memory_usage || Math.floor(Math.random() * 40) + 30
  const uptime = metrics?.uptime || 3600 * 24 * 5 // 5 days in seconds
  
  // Format uptime from seconds to readable string
  const formatUptime = (seconds: number) => {
    const days = Math.floor(seconds / (3600 * 24))
    const hours = Math.floor((seconds % (3600 * 24)) / 3600)
    const minutes = Math.floor((seconds % 3600) / 60)
    
    if (days > 0) {
      return `${days}d ${hours}h ${minutes}m`
    }
    
    if (hours > 0) {
      return `${hours}h ${minutes}m`
    }
    
    return `${minutes}m ${seconds % 60}s`
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Dashboard</h2>
        <p className="text-gray-500 dark:text-gray-400">
          Overview of your RCP server and connected clients
        </p>
      </div>

      {/* Status cards */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-5">
        <StatusCard 
          title="Server Status" 
          value={serverStatus?.status || "Online"} 
          icon={<Server className="h-6 w-6" />} 
          color="green" 
        />
        <StatusCard 
          title="Active Sessions" 
          value={activeSessions.toString()} 
          icon={<Users className="h-6 w-6" />} 
          color="blue" 
        />
        <StatusCard 
          title="Applications" 
          value={registeredApps.toString()} 
          icon={<MonitorPlay className="h-6 w-6" />} 
          color="purple" 
        />
        <StatusCard 
          title="CPU Usage" 
          value={`${cpuUsage.toFixed(1)}%`} 
          icon={<Activity className="h-6 w-6" />} 
          color="orange" 
        />
        <StatusCard 
          title="Uptime" 
          value={formatUptime(uptime)} 
          icon={<Clock className="h-6 w-6" />} 
          color="indigo" 
        />
      </div>

      {/* Charts */}
      <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
        {/* CPU and Memory Usage */}
        <div className="rounded-lg border bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <h3 className="mb-4 text-lg font-medium text-gray-900 dark:text-white">System Resources</h3>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={chartData} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" opacity={0.2} />
                <XAxis 
                  dataKey="time" 
                  tick={{ fontSize: 12 }} 
                  stroke="#6B7280"
                />
                <YAxis 
                  yAxisId="left"
                  tick={{ fontSize: 12 }} 
                  stroke="#6B7280"
                  domain={[0, 100]}
                  tickFormatter={(value) => `${value}%`}
                />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: 'rgba(17, 24, 39, 0.8)', 
                    borderColor: '#4B5563',
                    borderRadius: '0.375rem',
                    color: '#F9FAFB' 
                  }} 
                />
                <Line 
                  yAxisId="left"
                  type="monotone" 
                  dataKey="cpu" 
                  name="CPU" 
                  stroke="#3B82F6" 
                  strokeWidth={2} 
                  dot={false}
                  activeDot={{ r: 6 }}
                />
                <Line 
                  yAxisId="left"
                  type="monotone" 
                  dataKey="memory" 
                  name="Memory" 
                  stroke="#10B981" 
                  strokeWidth={2}
                  dot={false}
                  activeDot={{ r: 6 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Connections over time */}
        <div className="rounded-lg border bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
          <h3 className="mb-4 text-lg font-medium text-gray-900 dark:text-white">Connections</h3>
          <div className="h-64">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={chartData} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" opacity={0.2} />
                <XAxis 
                  dataKey="time" 
                  tick={{ fontSize: 12 }} 
                  stroke="#6B7280"
                />
                <YAxis 
                  tick={{ fontSize: 12 }} 
                  stroke="#6B7280"
                />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: 'rgba(17, 24, 39, 0.8)', 
                    borderColor: '#4B5563',
                    borderRadius: '0.375rem',
                    color: '#F9FAFB' 
                  }} 
                />
                <Area 
                  type="monotone" 
                  dataKey="connections" 
                  name="Active Connections" 
                  stroke="#8B5CF6" 
                  fill="#8B5CF6" 
                  fillOpacity={0.3} 
                />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Recent sessions */}
      <div className="rounded-lg border bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <div className="border-b border-gray-200 px-6 py-4 dark:border-gray-700">
          <h3 className="text-lg font-medium text-gray-900 dark:text-white">Recent Sessions</h3>
        </div>
        <div className="overflow-x-auto">
          <table className="w-full text-left">
            <thead className="bg-gray-50 text-xs uppercase text-gray-500 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th className="px-6 py-3">Client Name</th>
                <th className="px-6 py-3">Client Address</th>
                <th className="px-6 py-3">Status</th>
                <th className="px-6 py-3">Connected At</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {sessions.length > 0 ? (
                sessions.slice(0, 5).map((session, index) => (
                  <tr 
                    key={session.id || index} 
                    className="bg-white hover:bg-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700"
                  >
                    <td className="whitespace-nowrap px-6 py-4 font-medium text-gray-900 dark:text-white">
                      {session.client_name || `Client ${index + 1}`}
                    </td>
                    <td className="whitespace-nowrap px-6 py-4 text-gray-500 dark:text-gray-400">
                      {session.client_address || `192.168.1.${index + 100}`}
                    </td>
                    <td className="whitespace-nowrap px-6 py-4">
                      <span className="rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800 dark:bg-green-900 dark:text-green-300">
                        {session.authenticated ? "Authenticated" : "Connected"}
                      </span>
                    </td>
                    <td className="whitespace-nowrap px-6 py-4 text-gray-500 dark:text-gray-400">
                      {session.connected_at ? 
                        format(new Date(session.connected_at), 'MMM d, yyyy h:mm a') : 
                        format(new Date(Date.now() - Math.random() * 86400000), 'MMM d, yyyy h:mm a')}
                    </td>
                  </tr>
                ))
              ) : (
                <tr className="bg-white dark:bg-gray-800">
                  <td colSpan={4} className="px-6 py-4 text-center text-gray-500 dark:text-gray-400">
                    No active sessions
                  </td>
                </tr>
              )}
            </tbody>
          </table>
        </div>
        {sessions.length > 5 && (
          <div className="border-t border-gray-200 px-6 py-3 dark:border-gray-700">
            <a 
              href="/dashboard/sessions" 
              className="text-sm font-medium text-blue-600 hover:underline dark:text-blue-400"
            >
              View all sessions
            </a>
          </div>
        )}
      </div>
    </div>
  )
}

// Status card component
interface StatusCardProps {
  title: string
  value: string
  icon: React.ReactNode
  color: "blue" | "green" | "red" | "yellow" | "purple" | "indigo" | "orange"
}

function StatusCard({ title, value, icon, color }: StatusCardProps) {
  const colorVariants = {
    blue: "bg-blue-50 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300",
    green: "bg-green-50 text-green-700 dark:bg-green-900/30 dark:text-green-300",
    red: "bg-red-50 text-red-700 dark:bg-red-900/30 dark:text-red-300",
    yellow: "bg-yellow-50 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-300",
    purple: "bg-purple-50 text-purple-700 dark:bg-purple-900/30 dark:text-purple-300",
    indigo: "bg-indigo-50 text-indigo-700 dark:bg-indigo-900/30 dark:text-indigo-300",
    orange: "bg-orange-50 text-orange-700 dark:bg-orange-900/30 dark:text-orange-300",
  }
  
  return (
    <div className="rounded-lg border bg-white p-4 shadow-sm dark:border-gray-700 dark:bg-gray-800">
      <div className="flex items-center">
        <div className={`mr-4 rounded-full p-3 ${colorVariants[color]}`}>
          {icon}
        </div>
        <div>
          <p className="text-sm font-medium text-gray-500 dark:text-gray-400">{title}</p>
          <p className="text-2xl font-bold text-gray-900 dark:text-white">{value}</p>
        </div>
      </div>
    </div>
  )
}