"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { ArrowLeft, Loader2 } from "lucide-react"
import Link from "next/link"
import { createApplication } from "@/lib/api"
import { useToast } from "@/components/ui/use-toast"

export default function NewApplicationPage() {
  const router = useRouter()
  const { toast } = useToast()
  const [isSubmitting, setIsSubmitting] = useState(false)
  
  const [formData, setFormData] = useState({
    name: "",
    description: "",
    type: "native",
    command: "",
    working_directory: "",
    version: "",
    status: "inactive",
    launch_args: "",
    env_vars: ""
  })

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target
    setFormData(prev => ({ ...prev, [name]: value }))
  }

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setIsSubmitting(true)

    try {
      // Parse the launch args string to an array
      const launch_args = formData.launch_args
        ? formData.launch_args.split("\\n").map(arg => arg.trim()).filter(Boolean)
        : undefined

      // Parse the environment variables string to an object
      let env_vars: Record<string, string> | undefined = undefined
      if (formData.env_vars) {
        env_vars = {}
        const envLines = formData.env_vars.split("\\n")
        for (const line of envLines) {
          const [key, value] = line.split("=").map(part => part.trim())
          if (key && value) {
            env_vars[key] = value
          }
        }
      }

      const applicationData = {
        name: formData.name,
        description: formData.description,
        type: formData.type,
        command: formData.command,
        working_directory: formData.working_directory || undefined,
        version: formData.version || undefined,
        status: formData.status as "active" | "inactive",
        launch_args,
        env_vars
      }

      const response = await createApplication(applicationData)

      if (response.data) {
        toast({
          title: "Application created",
          description: `${formData.name} has been created successfully`,
        })
        router.push("/dashboard/applications")
      } else if (response.error) {
        toast({
          title: "Error creating application",
          description: response.error.message,
          variant: "destructive",
        })
      }
    } catch (error) {
      console.error("Failed to create application:", error)
      toast({
        title: "Error creating application",
        description: "Could not create the application",
        variant: "destructive",
      })
    } finally {
      setIsSubmitting(false)
    }
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-2">
        <Link 
          href="/dashboard/applications" 
          className="inline-flex items-center rounded-md border border-gray-300 bg-white px-3 py-2 text-sm font-medium text-gray-500 shadow-sm hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-800 dark:text-gray-300 dark:hover:bg-gray-700"
        >
          <ArrowLeft className="mr-2 h-4 w-4" />
          Back
        </Link>
        <h2 className="text-2xl font-bold text-gray-900 dark:text-white">Add New Application</h2>
      </div>

      <div className="rounded-lg border bg-white p-6 shadow-sm dark:border-gray-700 dark:bg-gray-800">
        <form onSubmit={handleSubmit} className="space-y-6">
          {/* Application Name */}
          <div>
            <label 
              htmlFor="name" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Application Name*
            </label>
            <input
              type="text"
              id="name"
              name="name"
              value={formData.name}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="My Application"
            />
          </div>
          
          {/* Application Description */}
          <div>
            <label 
              htmlFor="description" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Description*
            </label>
            <textarea
              id="description"
              name="description"
              value={formData.description}
              onChange={handleChange}
              required
              rows={3}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="A brief description of this application"
            ></textarea>
          </div>
          
          {/* Type */}
          <div>
            <label 
              htmlFor="type" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Type*
            </label>
            <select
              id="type"
              name="type"
              value={formData.type}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            >
              <option value="native">Native</option>
              <option value="web">Web</option>
              <option value="shell">Shell Script</option>
            </select>
          </div>
          
          {/* Command */}
          <div>
            <label 
              htmlFor="command" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Command*
            </label>
            <input
              type="text"
              id="command"
              name="command"
              value={formData.command}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="C:\\path\\to\\executable.exe or https://example.com"
            />
          </div>
          
          {/* Working Directory */}
          <div>
            <label 
              htmlFor="working_directory" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Working Directory
            </label>
            <input
              type="text"
              id="working_directory"
              name="working_directory"
              value={formData.working_directory}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="C:\\path\\to\\directory"
            />
          </div>
          
          {/* Version */}
          <div>
            <label 
              htmlFor="version" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Version
            </label>
            <input
              type="text"
              id="version"
              name="version"
              value={formData.version}
              onChange={handleChange}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="1.0.0"
            />
          </div>
          
          {/* Status */}
          <div>
            <label 
              htmlFor="status" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Status*
            </label>
            <select
              id="status"
              name="status"
              value={formData.status}
              onChange={handleChange}
              required
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
            >
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
            </select>
          </div>
          
          {/* Launch Arguments */}
          <div>
            <label 
              htmlFor="launch_args" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Launch Arguments
            </label>
            <textarea
              id="launch_args"
              name="launch_args"
              value={formData.launch_args}
              onChange={handleChange}
              rows={3}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="--arg1=value1\n--arg2=value2"
            ></textarea>
            <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
              Enter each argument on a new line
            </p>
          </div>
          
          {/* Environment Variables */}
          <div>
            <label 
              htmlFor="env_vars" 
              className="block text-sm font-medium text-gray-700 dark:text-gray-200"
            >
              Environment Variables
            </label>
            <textarea
              id="env_vars"
              name="env_vars"
              value={formData.env_vars}
              onChange={handleChange}
              rows={3}
              className="mt-1 block w-full rounded-md border border-gray-300 px-3 py-2 shadow-sm focus:border-blue-500 focus:outline-none focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
              placeholder="KEY1=value1\nKEY2=value2"
            ></textarea>
            <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
              Enter each variable in KEY=value format, one per line
            </p>
          </div>
          
          <div className="flex justify-end pt-4">
            <Link
              href="/dashboard/applications"
              className="mr-3 rounded-md border border-gray-300 bg-white px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:bg-gray-700 dark:text-gray-200 dark:hover:bg-gray-600"
            >
              Cancel
            </Link>
            <button
              type="submit"
              disabled={isSubmitting}
              className="inline-flex items-center rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-75"
            >
              {isSubmitting && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              Save Application
            </button>
          </div>
        </form>
      </div>
    </div>
  )
}