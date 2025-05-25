"use client"

import { useState } from "react"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Save, RefreshCcw, Server, ShieldCheck, Network, Download, Upload } from "lucide-react"
import { useToast } from "@/components/ui/use-toast"

export default function SettingsPage() {
  const { toast } = useToast()
  const [loading, setLoading] = useState(false)
  const [serverSettings, setServerSettings] = useState({
    port: 8765,
    host: "0.0.0.0",
    max_connections: 100,
    connection_timeout: 30,
    idle_timeout: 300,
    keepalive_interval: 60,
  })
  
  const [securitySettings, setSecuritySettings] = useState({
    require_authentication: true,
    tls_enabled: true,
    cert_path: "/etc/rcpd/certs/server.crt",
    key_path: "/etc/rcpd/certs/server.key",
    client_cert_required: false,
    allowed_ips: "",
    denied_ips: "",
  })
  
  const [loggingSettings, setLoggingSettings] = useState({
    log_level: "info",
    log_file: "/var/log/rcpd/server.log",
    max_log_size: 10,
    max_log_files: 5,
    log_format: "json",
  })

  const handleServerSettingsChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target
    setServerSettings(prev => ({
      ...prev,
      [name]: type === "number" ? parseInt(value) : value,
    }))
  }

  const handleSecuritySettingsChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value, type } = e.target
    setSecuritySettings(prev => ({
      ...prev,
      [name]: type === "checkbox" ? (e.target as HTMLInputElement).checked : value,
    }))
  }

  const handleLoggingSettingsChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target
    setLoggingSettings(prev => ({
      ...prev,
      [name]: type === "number" ? parseInt(value) : value,
    }))
  }

  const saveServerSettings = () => {
    setLoading(true)
    
    // API call would go here
    // For now, we'll just simulate a successful update
    setTimeout(() => {
      setLoading(false)
      toast({
        title: "Settings saved",
        description: "Server settings have been updated successfully.",
      })
    }, 1000)
  }

  const saveSecuritySettings = () => {
    setLoading(true)
    
    // API call would go here
    // For now, we'll just simulate a successful update
    setTimeout(() => {
      setLoading(false)
      toast({
        title: "Settings saved",
        description: "Security settings have been updated successfully.",
      })
    }, 1000)
  }

  const saveLoggingSettings = () => {
    setLoading(true)
    
    // API call would go here
    // For now, we'll just simulate a successful update
    setTimeout(() => {
      setLoading(false)
      toast({
        title: "Settings saved",
        description: "Logging settings have been updated successfully.",
      })
    }, 1000)
  }

  const downloadLogs = () => {
    toast({
      title: "Downloading logs",
      description: "Your logs are being compiled and will download shortly.",
    })
  }

  const restartServer = () => {
    if (confirm("Are you sure you want to restart the RCP daemon? All active connections will be terminated.")) {
      setLoading(true)
      
      // API call would go here
      // For now, we'll just simulate a successful restart
      setTimeout(() => {
        setLoading(false)
        toast({
          title: "Server restarted",
          description: "The RCP daemon has been restarted successfully.",
        })
      }, 2000)
    }
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Settings</h2>
        <p className="text-gray-500 dark:text-gray-400">
          Configure server parameters and system settings
        </p>
      </div>

      <div className="rounded-lg border bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <Tabs defaultValue="server">
          <div className="border-b border-gray-200 dark:border-gray-700">
            <TabsList className="flex h-auto rounded-none border-b border-gray-200 bg-transparent p-0 dark:border-gray-700">
              <TabsTrigger
                value="server"
                className="inline-flex items-center border-b-2 border-transparent px-4 py-3 text-center text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700 data-[state=active]:border-blue-500 data-[state=active]:text-blue-600 dark:text-gray-400 dark:hover:text-gray-300 dark:data-[state=active]:border-blue-500 dark:data-[state=active]:text-blue-400"
              >
                <Server className="mr-2 h-4 w-4" />
                Server
              </TabsTrigger>
              <TabsTrigger
                value="security"
                className="inline-flex items-center border-b-2 border-transparent px-4 py-3 text-center text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700 data-[state=active]:border-blue-500 data-[state=active]:text-blue-600 dark:text-gray-400 dark:hover:text-gray-300 dark:data-[state=active]:border-blue-500 dark:data-[state=active]:text-blue-400"
              >
                <ShieldCheck className="mr-2 h-4 w-4" />
                Security
              </TabsTrigger>
              <TabsTrigger
                value="logging"
                className="inline-flex items-center border-b-2 border-transparent px-4 py-3 text-center text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700 data-[state=active]:border-blue-500 data-[state=active]:text-blue-600 dark:text-gray-400 dark:hover:text-gray-300 dark:data-[state=active]:border-blue-500 dark:data-[state=active]:text-blue-400"
              >
                <Upload className="mr-2 h-4 w-4" />
                Logging
              </TabsTrigger>
            </TabsList>
          </div>
          
          {/* Server Settings Tab */}
          <TabsContent value="server" className="p-6">
            <form
              onSubmit={(e) => {
                e.preventDefault()
                saveServerSettings()
              }}
              className="space-y-6"
            >
              <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                <div>
                  <label htmlFor="port" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Port
                  </label>
                  <input
                    type="number"
                    id="port"
                    name="port"
                    value={serverSettings.port}
                    onChange={handleServerSettingsChange}
                    min="1"
                    max="65535"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Port the RCP daemon will listen on
                  </p>
                </div>

                <div>
                  <label htmlFor="host" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Host
                  </label>
                  <input
                    type="text"
                    id="host"
                    name="host"
                    value={serverSettings.host}
                    onChange={handleServerSettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Network interface to bind to (0.0.0.0 for all interfaces)
                  </p>
                </div>

                <div>
                  <label htmlFor="max_connections" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Max Connections
                  </label>
                  <input
                    type="number"
                    id="max_connections"
                    name="max_connections"
                    value={serverSettings.max_connections}
                    onChange={handleServerSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Maximum number of simultaneous connections
                  </p>
                </div>

                <div>
                  <label htmlFor="connection_timeout" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Connection Timeout (seconds)
                  </label>
                  <input
                    type="number"
                    id="connection_timeout"
                    name="connection_timeout"
                    value={serverSettings.connection_timeout}
                    onChange={handleServerSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Time before initial connection times out
                  </p>
                </div>

                <div>
                  <label htmlFor="idle_timeout" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Idle Timeout (seconds)
                  </label>
                  <input
                    type="number"
                    id="idle_timeout"
                    name="idle_timeout"
                    value={serverSettings.idle_timeout}
                    onChange={handleServerSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Time before idle connection is closed
                  </p>
                </div>

                <div>
                  <label htmlFor="keepalive_interval" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Keepalive Interval (seconds)
                  </label>
                  <input
                    type="number"
                    id="keepalive_interval"
                    name="keepalive_interval"
                    value={serverSettings.keepalive_interval}
                    onChange={handleServerSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    How often to send keepalive messages
                  </p>
                </div>
              </div>

              <div className="flex justify-between border-t border-gray-200 pt-5 dark:border-gray-700">
                <button
                  type="button"
                  onClick={restartServer}
                  disabled={loading}
                  className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700"
                >
                  <RefreshCcw className="mr-2 h-4 w-4" />
                  Restart Server
                </button>
                <button
                  type="submit"
                  disabled={loading}
                  className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
                >
                  <Save className="mr-2 h-4 w-4" />
                  {loading ? "Saving..." : "Save Settings"}
                </button>
              </div>
            </form>
          </TabsContent>

          {/* Security Settings Tab */}
          <TabsContent value="security" className="p-6">
            <form
              onSubmit={(e) => {
                e.preventDefault()
                saveSecuritySettings()
              }}
              className="space-y-6"
            >
              <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                <div>
                  <div className="flex items-center">
                    <input
                      type="checkbox"
                      id="require_authentication"
                      name="require_authentication"
                      checked={securitySettings.require_authentication}
                      onChange={handleSecuritySettingsChange}
                      className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700"
                    />
                    <label htmlFor="require_authentication" className="ml-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Require Authentication
                    </label>
                  </div>
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Require clients to authenticate before accessing services
                  </p>
                </div>

                <div>
                  <div className="flex items-center">
                    <input
                      type="checkbox"
                      id="tls_enabled"
                      name="tls_enabled"
                      checked={securitySettings.tls_enabled}
                      onChange={handleSecuritySettingsChange}
                      className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700"
                    />
                    <label htmlFor="tls_enabled" className="ml-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Enable TLS/SSL
                    </label>
                  </div>
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Encrypt all communications using TLS/SSL
                  </p>
                </div>

                <div>
                  <label htmlFor="cert_path" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Certificate Path
                  </label>
                  <input
                    type="text"
                    id="cert_path"
                    name="cert_path"
                    value={securitySettings.cert_path}
                    onChange={handleSecuritySettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Path to TLS certificate file
                  </p>
                </div>

                <div>
                  <label htmlFor="key_path" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Key Path
                  </label>
                  <input
                    type="text"
                    id="key_path"
                    name="key_path"
                    value={securitySettings.key_path}
                    onChange={handleSecuritySettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Path to TLS key file
                  </p>
                </div>

                <div>
                  <div className="flex items-center">
                    <input
                      type="checkbox"
                      id="client_cert_required"
                      name="client_cert_required"
                      checked={securitySettings.client_cert_required}
                      onChange={handleSecuritySettingsChange}
                      className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700"
                    />
                    <label htmlFor="client_cert_required" className="ml-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
                      Require Client Certificates
                    </label>
                  </div>
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Require clients to present valid certificates
                  </p>
                </div>
              </div>

              <div>
                <label htmlFor="allowed_ips" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Allowed IP Addresses
                </label>
                <textarea
                  id="allowed_ips"
                  name="allowed_ips"
                  value={securitySettings.allowed_ips}
                  onChange={handleSecuritySettingsChange}
                  rows={3}
                  className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  placeholder="192.168.1.0/24, 10.0.0.5"
                />
                <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                  List of IP addresses/ranges allowed to connect (comma separated, leave empty for all)
                </p>
              </div>

              <div>
                <label htmlFor="denied_ips" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Denied IP Addresses
                </label>
                <textarea
                  id="denied_ips"
                  name="denied_ips"
                  value={securitySettings.denied_ips}
                  onChange={handleSecuritySettingsChange}
                  rows={3}
                  className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  placeholder="192.168.1.100, 10.0.0.50/32"
                />
                <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                  List of IP addresses/ranges denied from connecting (comma separated, leave empty for none)
                </p>
              </div>

              <div className="flex justify-end border-t border-gray-200 pt-5 dark:border-gray-700">
                <button
                  type="submit"
                  disabled={loading}
                  className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
                >
                  <Save className="mr-2 h-4 w-4" />
                  {loading ? "Saving..." : "Save Security Settings"}
                </button>
              </div>
            </form>
          </TabsContent>

          {/* Logging Settings Tab */}
          <TabsContent value="logging" className="p-6">
            <form
              onSubmit={(e) => {
                e.preventDefault()
                saveLoggingSettings()
              }}
              className="space-y-6"
            >
              <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
                <div>
                  <label htmlFor="log_level" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Log Level
                  </label>
                  <select
                    id="log_level"
                    name="log_level"
                    value={loggingSettings.log_level}
                    onChange={handleLoggingSettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  >
                    <option value="trace">Trace</option>
                    <option value="debug">Debug</option>
                    <option value="info">Info</option>
                    <option value="warn">Warning</option>
                    <option value="error">Error</option>
                  </select>
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Verbosity level of logs
                  </p>
                </div>

                <div>
                  <label htmlFor="log_file" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Log File Path
                  </label>
                  <input
                    type="text"
                    id="log_file"
                    name="log_file"
                    value={loggingSettings.log_file}
                    onChange={handleLoggingSettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Path to write log files
                  </p>
                </div>

                <div>
                  <label htmlFor="max_log_size" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Max Log Size (MB)
                  </label>
                  <input
                    type="number"
                    id="max_log_size"
                    name="max_log_size"
                    value={loggingSettings.max_log_size}
                    onChange={handleLoggingSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Maximum size of a single log file before rotation
                  </p>
                </div>

                <div>
                  <label htmlFor="max_log_files" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Max Log Files
                  </label>
                  <input
                    type="number"
                    id="max_log_files"
                    name="max_log_files"
                    value={loggingSettings.max_log_files}
                    onChange={handleLoggingSettingsChange}
                    min="1"
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Number of log files to keep before deleting older files
                  </p>
                </div>

                <div>
                  <label htmlFor="log_format" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                    Log Format
                  </label>
                  <select
                    id="log_format"
                    name="log_format"
                    value={loggingSettings.log_format}
                    onChange={handleLoggingSettingsChange}
                    className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                  >
                    <option value="json">JSON</option>
                    <option value="text">Plain Text</option>
                  </select>
                  <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                    Format for log output
                  </p>
                </div>
              </div>

              <div className="flex justify-between border-t border-gray-200 pt-5 dark:border-gray-700">
                <button
                  type="button"
                  onClick={downloadLogs}
                  className="inline-flex items-center rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 dark:border-gray-600 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700"
                >
                  <Download className="mr-2 h-4 w-4" />
                  Download Logs
                </button>
                <button
                  type="submit"
                  disabled={loading}
                  className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50"
                >
                  <Save className="mr-2 h-4 w-4" />
                  {loading ? "Saving..." : "Save Logging Settings"}
                </button>
              </div>
            </form>
          </TabsContent>
        </Tabs>
      </div>
    </div>
  )
}