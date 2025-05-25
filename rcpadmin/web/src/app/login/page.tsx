"use client"

import { useState } from "react"
import { useRouter } from "next/navigation"
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"
import * as z from "zod"
import { login } from "@/lib/api"
import { useToast } from "@/components/ui/use-toast"

const loginSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
})

export default function LoginPage() {
  const [isLoading, setIsLoading] = useState(false)
  const router = useRouter()
  const { toast } = useToast()
  
  const form = useForm<z.infer<typeof loginSchema>>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      username: "",
      password: "",
    },
  })

  async function onSubmit(values: z.infer<typeof loginSchema>) {
    setIsLoading(true)
    
    const response = await login(values.username, values.password)
    
    setIsLoading(false)
    
    if (response.error) {
      toast({
        title: "Authentication failed",
        description: response.error.message,
        variant: "destructive",
      })
      return
    }
    
    // Save token and redirect to dashboard
    localStorage.setItem("authToken", response.data.token)
    localStorage.setItem("userData", JSON.stringify(response.data.user))
    
    toast({
      title: "Login successful",
      description: "Welcome to RCP Admin Dashboard",
    })
    
    router.push("/dashboard")
  }

  return (
    <div className="flex min-h-screen flex-col items-center justify-center bg-gradient-to-r from-blue-900 to-slate-900 p-4">
      <div className="w-full max-w-md space-y-8 rounded-lg bg-white p-8 shadow-xl dark:bg-slate-900">
        <div className="flex flex-col items-center space-y-2 text-center">
          <h1 className="text-4xl font-bold text-slate-900 dark:text-white">RCP Admin</h1>
          <h2 className="text-xl text-slate-600 dark:text-slate-400">
            Rust/Remote Control Protocol
          </h2>
          <p className="text-sm text-slate-500 dark:text-slate-400">
            Sign in to access the administration dashboard
          </p>
        </div>

        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-6">
          <div className="space-y-2">
            <label 
              htmlFor="username" 
              className="text-sm font-medium text-slate-700 dark:text-slate-300"
            >
              Username
            </label>
            <input
              id="username"
              type="text"
              disabled={isLoading}
              {...form.register("username")}
              className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white dark:placeholder-slate-500"
            />
            {form.formState.errors.username && (
              <p className="text-sm text-red-500">{form.formState.errors.username.message}</p>
            )}
          </div>
          
          <div className="space-y-2">
            <div className="flex items-center justify-between">
              <label 
                htmlFor="password" 
                className="text-sm font-medium text-slate-700 dark:text-slate-300"
              >
                Password
              </label>
              <a href="#" className="text-xs text-primary hover:underline">
                Forgot password?
              </a>
            </div>
            <input
              id="password"
              type="password"
              disabled={isLoading}
              {...form.register("password")}
              className="w-full rounded-md border border-slate-300 bg-white px-3 py-2 text-slate-900 placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary dark:border-slate-700 dark:bg-slate-800 dark:text-white dark:placeholder-slate-500"
            />
            {form.formState.errors.password && (
              <p className="text-sm text-red-500">{form.formState.errors.password.message}</p>
            )}
          </div>

          <button
            disabled={isLoading}
            type="submit"
            className="w-full rounded-md bg-primary px-4 py-2 text-white hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-primary disabled:opacity-50"
          >
            {isLoading ? "Signing in..." : "Sign In"}
          </button>
        </form>
        
        <div className="text-center text-xs text-slate-500 dark:text-slate-400">
          <p>Default admin login: admin / rcpadmin</p>
          <p className="mt-4">© {new Date().getFullYear()} Devstroop Technologies</p>
        </div>
      </div>
    </div>
  )
}