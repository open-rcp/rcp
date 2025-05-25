"use client"

import { useState, useEffect } from "react"
import { useRouter } from "next/navigation"
import Link from "next/link"
import { ArrowLeft, Save } from "lucide-react"
import { getApplication, updateApplication } from "@/lib/api"
import { useToast } from "@/components/ui/use-toast"

interface ApplicationFormData {
  id: string
  name: string
  path: string
  arguments?: string
  working_dir?: string
  enabled: boolean
  icon?: string
  description?: string
}

export default function EditApplicationPage({ params }: { params: { id: string } }) {
  const router = useRouter()
  const { toast } = useToast()
  const [isLoading, setIsLoading] = useState(false)
  const [isFetching, setIsFetching] = useState(true)
  const [formData, setFormData] = useState<ApplicationFormData>({
    id: "",
    name: "",
    path: "",
    arguments: "",
    working_dir: "",
    enabled: true,
    icon: "",
    description: "",
  })

  useEffect(() => {
    async function fetchApplication() {
      try {
        const response = await getApplication(params.id)
        
        if (response.error) {
          toast({
            title: "Error",
            description: response.error.message,
            variant: "destructive",
          })
          router.push("/dashboard/applications")
          return
        }
        
        // Convert arguments array back to string for form
        const app = response.data
        setFormData({
          ...app,
          arguments: app.arguments ? app.arguments.join(", ") : "",
        })
        
      } catch (error) {
        toast({
          title: "Error",
          description: "Failed to fetch application",
          variant: "destructive",
        })
        router.push("/dashboard/applications")
      } finally {
        setIsFetching(false)
      }
    }

    fetchApplication()
  }, [params.id, router, toast])

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value, type } = e.target
    
    setFormData(prev => ({
      ...prev,
      [name]: type === "checkbox" 
        ? (e.target as HTMLInputElement).checked 
        : value
    }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsLoading(true)

    try {
      // Transform arguments string to array
      const applicationData = {
        ...formData,
        arguments: formData.arguments ? formData.arguments.split(',').map(arg => arg.trim()) : undefined,
      }

      const response = await updateApplication(params.id, applicationData)
      
      if (response.error) {
        toast({
          title: "Error",
          description: response.error.message,
          variant: "destructive",
        })
        setIsLoading(false)
        return
      }
      
      toast({
        title: "Success",
        description: "Application updated successfully",
      })
      
      // Navigate back to applications list
      router.push("/dashboard/applications")
    } catch (error) {
      toast({
        title: "Error",
        description: "Failed to update application",
        variant: "destructive",
      })
      setIsLoading(false)
    }
  }

  if (isFetching) {
    return (
      <div className="flex h-full items-center justify-center">
        <div className="text-center">
          <div className="h-8 w-8 animate-spin rounded-full border-4 border-b-transparent"></div>
          <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">Loading application...</p>
        </div>
      </div>
    )
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Link 
            href="/dashboard/applications" 
            className="inline-flex items-center rounded-md border border-gray-300 bg-white px-3 py-2 text-sm font-medium text-gray-700 shadow-sm hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-white dark:hover:bg-gray-700"
          >
            <ArrowLeft className="mr-2 h-4 w-4" />
            Back
          </Link>
          <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Edit Application</h2>
        </div>
        
        <button
          type="submit"
          form="application-form"
          disabled={isLoading}
          className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:pointer-events-none"
        >
          <Save className="mr-2 h-4 w-4" />
          {isLoading ? "Saving..." : "Save Changes"}
        </button>
      </div>

      <div className="rounded-lg border bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <form id="application-form" onSubmit={handleSubmit} className="space-y-6">
          <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
            {/* Application ID - readonly as it's the identifier */}
            <div>
              <label htmlFor="id" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Application ID <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="id"
                name="id"
                value={formData.id}
                disabled
                className="mt-1 block w-full rounded-md border border-gray-300 bg-gray-100 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-400 sm:text-sm"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Unique identifier (cannot be changed)
              </p>
            </div>

            {/* Application Name */}
            <div>
              <label htmlFor="name" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Display Name <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="name"
                name="name"
                value={formData.name}
                onChange={handleChange}
                required
                className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                User-friendly name displayed to users
              </p>
            </div>

            {/* Executable Path */}
            <div>
              <label htmlFor="path" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Executable Path <span className="text-red-500">*</span>
              </label>
              <input
                type="text"
                id="path"
                name="path"
                value={formData.path}
                onChange={handleChange}
                required
                className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Full path to the executable
              </p>
            </div>

            {/* Arguments */}
            <div>
              <label htmlFor="arguments" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Arguments
              </label>
              <input
                type="text"
                id="arguments"
                name="arguments"
                value={formData.arguments}
                onChange={handleChange}
                className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
                placeholder="--incognito, --disable-gpu"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Comma-separated list of arguments to pass to the executable
              </p>
            </div>

            {/* Working Directory */}
            <div>
              <label htmlFor="working_dir" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Working Directory
              </label>
              <input
                type="text"
                id="working_dir"
                name="working_dir"
                value={formData.working_dir || ""}
                onChange={handleChange}
                className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Directory to start the application in (optional)
              </p>
            </div>

            {/* Icon Path */}
            <div>
              <label htmlFor="icon" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                Icon Path
              </label>
              <input
                type="text"
                id="icon"
                name="icon"
                value={formData.icon || ""}
                onChange={handleChange}
                className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
              />
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Path or URL to application icon (optional)
              </p>
            </div>

            {/* Enabled Status */}
            <div>
              <div className="flex items-center">
                <input
                  type="checkbox"
                  id="enabled"
                  name="enabled"
                  checked={formData.enabled}
                  onChange={handleChange}
                  className="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700"
                />
                <label htmlFor="enabled" className="ml-2 block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Enabled
                </label>
              </div>
              <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
                Whether this application is available for launching
              </p>
            </div>
          </div>

          {/* Description */}
          <div>
            <label htmlFor="description" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
              Description
            </label>
            <textarea
              id="description"
              name="description"
              value={formData.description || ""}
              onChange={handleChange}
              rows={3}
              className="mt-1 block w-full rounded-md border border-gray-300 bg-white px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white sm:text-sm"
            />
          </div>
        </form>
      </div>
    </div>
  )
}