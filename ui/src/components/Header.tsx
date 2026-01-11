import { Link } from '@tanstack/react-router'
import { MessageSquare } from 'lucide-react'

export default function Header() {
  return (
    <header className="h-16 px-4 flex items-center border-b bg-background">
      <Link to="/" className="flex items-center gap-2">
        <MessageSquare className="h-6 w-6" />
        <h1 className="text-xl font-semibold">Chat</h1>
      </Link>
    </header>
  )
}
