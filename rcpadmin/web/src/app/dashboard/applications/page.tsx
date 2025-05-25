"use client"

import { useState, useEffect } from "react"
import { Plus, Search, Package, Edit, Trash2, Play, MoreHorizontal } from "lucide-react"
import { getApplications, deleteApplication } from "@/lib/api"
import Link from "next/link"
import { useToast } from "@/components/ui/use-toast"
import { format } from "date-fns"

export default function ApplicationsPage() {
  const [applications, setApplications] = useState<any[]>([])
  const [loading, setLoading] = useState(true)
  const [searchQuery, setSearchQuery] = useState("")
  const { toast } = useToast()

  // Fetch applications data
  useEffect(() => {
    async function fetchApplications() {
      setLoading(true)
      try {
        const response = await getApplications()
        if (response.data) {
          setApplications(response.data)
        } else if (response.error) {
          toast({
            title: "Error fetching applications",
            description: response.error.message,
            variant: "destructive",
          })
        }
      } catch (error) {
        console.error("Failed to fetch applications:", error)
        toast({
          title: "Error fetching applications",
          description: "Could not load applications data",
          variant: "destructive",
        })
      } finally {
        setLoading(false)
      }
    }

    fetchApplications()
  }, [toast])

  // Handle application deletion
  const handleDeleteApplication = async (id: string) => {
    if (confirm("Are you sure you want to delete this application?")) {
      try {
        const response = await deleteApplication(id)
        if (response.data?.success) {
          toast({
            title: "Application deleted",
            description: "The application has been deleted successfully",
          })
          // Update the local state to remove the deleted application
          setApplications(prevApps => prevApps.filter(app => app.id !== id))
        } else if (response.error) {
          toast({
            title: "Error deleting application",
            description: response.error.message,
            variant: "destructive",
          })
        }
      } catch (error) {
        console.error("Failed to delete application:", error)
        toast({
          title: "Error deleting application",
          description: "Could not delete the application",
          variant: "destructive",
        })
      }
    }
  }

  // Filter applications based on search query
  const filteredApplications = applications.filter(app => {
    const query = searchQuery.toLowerCase()
    return (
      app.name.toLowerCase().includes(query) ||
      app.description.toLowerCase().includes(query) ||
      app.type.toLowerCase().includes(query)
    )
  })

  return (
    <div className="space-y-6">
      <div className="flex flex-col justify-between gap-4 sm:flex-row sm:items-center">
        <div>
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Applications</h2>
          <p className="text-gray-500 dark:text-gray-400">
            Manage applications that can be launched via RCP
          </p>
        </div>
        <Link
          href="/dashboard/applications/new"
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
        >
          <Plus className="mr-2 h-4 w-4" />
          Add Application
        </Link>
      </div>

      {/* Search bar */}
      <div className="relative">
        <div className="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3">
          <Search className="h-5 w-5 text-gray-400" />
        </div>
        <input
          type="text"
          placeholder="Search applications..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="block w-full rounded-md border border-gray-300 bg-white p-2 pl-10 text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
        />
      </div>

      {/* Applications table */}
      <div className="rounded-lg border bg-white shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <div className="overflow-x-auto">
          <table className="w-full text-left">
            <thead className="bg-gray-50 text-xs uppercase text-gray-500 dark:bg-gray-700 dark:text-gray-400">
              <tr>
                <th className="px-6 py-3">Name</th>
                <th className="px-6 py-3">Type</th>
                <th className="px-6 py-3">Command</th>
                <th className="px-6 py-3">Status</th>
                <th className="px-6 py-3">Last Updated</th>
                <th className="px-6 py-3 text-right">Actions</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
              {loading ? (
                <tr>
                  <td colSpan={6} className="px-6 py-12 text-center">
                    <div className="flex justify-center">
                      <div className="h-8 w-8 animate-spin rounded-full border-4 border-blue-500 border-b-transparent"></div>
                    </div>
                    <p className="mt-2 text-gray-500 dark:text-gray-400">
                      Loading applications...
                    </p>
                  </td>
                </tr>
              ) : filteredApplications.length === 0 ? (
                <tr>
                  <td colSpan={6} className="px-6 py-12 text-center">
                    <div className="flex justify-center">
                      <Package className="h-10 w-10 text-gray-400" />
                    </div>
                    <p className="mt-2 text-gray-500 dark:text-gray-400">
                      {searchQuery ? "No applications match your search" : "No applications found"}
                    </p>
                    <Link
                      href="/dashboard/applications/new"
                      className="mt-4 inline-flex items-center rounded-md bg-blue-600 px-3 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                      <Plus className="mr-2 h-4 w-4" />
                      Add your first application
                    </Link>
                  </td>
                </tr>
              ) : (
                filteredApplications.map((app) => (
                  <tr key={app.id} className="bg-white hover:bg-gray-50 dark:bg-gray-800 dark:hover:bg-gray-700">
                    <td className="whitespace-nowrap px-6 py-4 font-medium text-gray-900 dark:text-white">
                      {app.name}
                    </td>
                    <td className="px-6 py-4 text-gray-500 dark:text-gray-400">
                      {app.type}
                    </td>
                    <td className="px-6 py-4">
                      <div className="max-w-xs truncate text-gray-500 dark:text-gray-400">
                        {app.command}
                      </div>
                    </td>
                    <td className="px-6 py-4">
                      <span
                        className={`inline-flex rounded-full px-2.5 py-0.5 text-xs font-medium ${
                          app.status === "active"
                            ? "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
                            : "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
                        }`}
                      >
                        {app.status}
                      </span>
                    </td>
                    <td className="px-6 py-4 text-gray-500 dark:text-gray-400">
                      {app.updated_at ? format(new Date(app.updated_at), "MMM d, yyyy") : "—"}
                    </td>
                    <td className="whitespace-nowrap px-6 py-4 text-right">
                      <div className="flex items-center justify-end space-x-2">
                        <Link
                          href={`/dashboard/applications/${app.id}`}
                          className="rounded p-1 text-gray-500 hover:bg-gray-100 hover:text-gray-900 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-white"
                          title="Edit"
                        >
                          <Edit className="h-5 w-5" />
                        </Link>
                        <button
                          onClick={() => handleDeleteApplication(app.id)}
                          className="rounded p-1 text-gray-500 hover:bg-gray-100 hover:text-red-600 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-red-500"
                          title="Delete"
                        >
                          <Trash2 className="h-5 w-5" />
                        </button>
                      </div>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}