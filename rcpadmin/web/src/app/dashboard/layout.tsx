"use client"

import { useEffect, useState } from "react"
import { usePathname, useRouter } from "next/navigation"
import Link from "next/link"
import { 
  LayoutDashboard, 
  Server, 
  MonitorPlay, 
  Users, 
  Settings, 
  LogOut,
  Menu,
  X,
  Moon,
  Sun
} from "lucide-react"
import { useTheme } from "next-themes"
import { cn } from "@/lib/utils"

interface SidebarNavProps {
  items: {
    title: string
    href: string
    icon: React.ReactNode
    subItems?: { title: string; href: string }[]
  }[]
}

interface UserData {
  id: string
  username: string
  email: string
  role: string
}

export default function DashboardLayout({ children }: { children: React.ReactNode }) {
  const [mounted, setMounted] = useState(false)
  const [sidebarOpen, setSidebarOpen] = useState(false)
  const [userData, setUserData] = useState<UserData | null>(null)
  const router = useRouter()
  const pathname = usePathname()
  
  useEffect(() => {
    // Check if logged in when component mounts
    const authToken = localStorage.getItem("authToken")
    const userDataStr = localStorage.getItem("userData")
    
    if (!authToken) {
      router.push("/login")
      return
    }
    
    try {
      if (userDataStr) {
        const parsedUserData = JSON.parse(userDataStr)
        setUserData(parsedUserData)
      }
    } catch (error) {
      console.error("Failed to parse user data", error)
      router.push("/login")
    }
    
    setMounted(true)
  }, [router])

  const handleLogout = () => {
    localStorage.removeItem("authToken")
    localStorage.removeItem("userData")
    router.push("/login")
  }

  // Only render UI if we've checked authentication
  if (!mounted) {
    return null
  }

  const sidebarItems = [
    {
      title: "Overview",
      href: "/dashboard",
      icon: <LayoutDashboard className="h-5 w-5" />,
    },
    {
      title: "Servers",
      href: "/dashboard/servers",
      icon: <Server className="h-5 w-5" />,
    },
    {
      title: "Applications",
      href: "/dashboard/applications",
      icon: <MonitorPlay className="h-5 w-5" />,
    },
    {
      title: "Users",
      href: "/dashboard/users",
      icon: <Users className="h-5 w-5" />,
    },
    {
      title: "Settings",
      href: "/dashboard/settings",
      icon: <Settings className="h-5 w-5" />,
    },
  ]

  return (
    <div className="flex h-screen overflow-hidden bg-gray-100 dark:bg-gray-900">
      {/* Sidebar for desktop */}
      <div className={cn(
        "fixed inset-y-0 left-0 z-20 w-64 transform bg-white transition-transform duration-300 ease-in-out dark:bg-gray-800 md:relative md:translate-x-0",
        sidebarOpen ? "translate-x-0" : "-translate-x-full"
      )}>
        <div className="flex h-full flex-col">
          {/* Sidebar header */}
          <div className="flex h-16 items-center justify-between px-4 md:justify-center">
            <span className="text-xl font-bold text-gray-800 dark:text-white">RCP Admin</span>
            <button 
              onClick={() => setSidebarOpen(false)}
              className="rounded p-1 text-gray-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700 md:hidden"
            >
              <X className="h-6 w-6" />
            </button>
          </div>
          
          {/* User info */}
          <div className="border-b border-gray-200 px-4 py-3 dark:border-gray-700">
            <p className="text-sm font-medium text-gray-900 dark:text-white">{userData?.username}</p>
            <p className="text-xs text-gray-500 dark:text-gray-400">{userData?.email}</p>
            <p className="mt-1 rounded-full bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900 dark:text-blue-300">
              {userData?.role}
            </p>
          </div>
          
          {/* Sidebar content */}
          <div className="flex-1 overflow-y-auto p-4">
            <SidebarNav items={sidebarItems} />
          </div>
          
          {/* Sidebar footer */}
          <div className="border-t border-gray-200 p-4 dark:border-gray-700">
            <button
              onClick={handleLogout}
              className="flex w-full items-center rounded-md px-3 py-2 text-sm font-medium text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              <LogOut className="mr-2 h-4 w-4" />
              <span>Log out</span>
            </button>
          </div>
        </div>
      </div>
      
      <div className="flex flex-1 flex-col overflow-hidden">
        {/* Top navbar */}
        <header className="flex h-16 items-center justify-between bg-white px-4 shadow dark:bg-gray-800">
          <button
            onClick={() => setSidebarOpen(true)}
            className="rounded p-1 text-gray-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700 md:hidden"
          >
            <Menu className="h-6 w-6" />
          </button>
          
          <div className="flex items-center space-x-4">
            <ThemeToggle />
          </div>
        </header>
        
        {/* Main content */}
        <main className="flex-1 overflow-y-auto p-4 md:p-6">
          {children}
        </main>
      </div>
    </div>
  )
}

function SidebarNav({ items }: SidebarNavProps) {
  const pathname = usePathname()
  
  return (
    <nav className="space-y-1">
      {items.map((item) => {
        const isActive = pathname === item.href
        
        return (
          <Link
            key={item.href}
            href={item.href}
            className={cn(
              "flex items-center rounded-md px-3 py-2 text-sm font-medium",
              isActive 
                ? "bg-blue-50 text-blue-700 dark:bg-blue-900/50 dark:text-blue-300"
                : "text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
            )}
          >
            <span className="mr-3">{item.icon}</span>
            <span>{item.title}</span>
          </Link>
        )
      })}
    </nav>
  )
}

function ThemeToggle() {
  const { theme, setTheme } = useTheme()
  
  return (
    <button
      onClick={() => setTheme(theme === "light" ? "dark" : "light")}
      className="rounded-md p-2 text-gray-600 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700"
    >
      {theme === "light" ? (
        <Moon className="h-5 w-5" />
      ) : (
        <Sun className="h-5 w-5" />
      )}
    </button>
  )
}