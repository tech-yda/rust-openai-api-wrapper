import { createFileRoute } from '@tanstack/react-router'
import { useState, useEffect, useCallback } from 'react'
import { ScrollArea } from '@/components/ui/scroll-area'
import {
  ChatMessage,
  ChatInput,
  SessionList,
  SystemPromptDialog,
} from '@/components/chat'
import { api, type Session, type Message } from '@/lib/api'
import { cn } from '@/lib/utils'

export const Route = createFileRoute('/')({ component: ChatPage })

function ChatPage() {
  const [sessions, setSessions] = useState<Session[]>([])
  const [currentSessionId, setCurrentSessionId] = useState<string | null>(null)
  const [messages, setMessages] = useState<Message[]>([])
  const [isLoading, setIsLoading] = useState(false)
  const [showNewSessionDialog, setShowNewSessionDialog] = useState(false)

  // Load session data when session changes
  useEffect(() => {
    if (currentSessionId) {
      api.getSession(currentSessionId).then((data) => {
        setMessages(data.messages)
      })
    } else {
      setMessages([])
    }
  }, [currentSessionId])

  const handleNewSession = useCallback(async (systemPrompt?: string) => {
    try {
      const session = await api.createSession({ system_prompt: systemPrompt })
      setSessions((prev) => [
        { id: session.id, system_prompt: session.system_prompt, created_at: session.created_at },
        ...prev,
      ])
      setCurrentSessionId(session.id)
      setMessages([])
    } catch (error) {
      console.error('Failed to create session:', error)
    }
  }, [])

  const handleDeleteSession = useCallback(
    async (id: string) => {
      try {
        await api.deleteSession(id)
        setSessions((prev) => prev.filter((s) => s.id !== id))
        if (currentSessionId === id) {
          setCurrentSessionId(null)
          setMessages([])
        }
      } catch (error) {
        console.error('Failed to delete session:', error)
      }
    },
    [currentSessionId]
  )

  const handleSendMessage = useCallback(
    async (content: string) => {
      if (!currentSessionId) {
        // Create a new session first
        const session = await api.createSession({})
        setSessions((prev) => [
          { id: session.id, system_prompt: session.system_prompt, created_at: session.created_at },
          ...prev,
        ])
        setCurrentSessionId(session.id)

        // Send message to new session
        setIsLoading(true)
        try {
          const userMessage: Message = {
            id: crypto.randomUUID(),
            session_id: session.id,
            role: 'user',
            content,
            created_at: new Date().toISOString(),
          }
          setMessages([userMessage])

          const response = await api.sendMessage(session.id, content)
          const assistantMessage: Message = {
            id: crypto.randomUUID(),
            session_id: session.id,
            role: 'assistant',
            content: response.response,
            created_at: new Date().toISOString(),
          }
          setMessages((prev) => [...prev, assistantMessage])
        } finally {
          setIsLoading(false)
        }
        return
      }

      setIsLoading(true)
      try {
        // Optimistically add user message
        const userMessage: Message = {
          id: crypto.randomUUID(),
          session_id: currentSessionId,
          role: 'user',
          content,
          created_at: new Date().toISOString(),
        }
        setMessages((prev) => [...prev, userMessage])

        const response = await api.sendMessage(currentSessionId, content)
        const assistantMessage: Message = {
          id: crypto.randomUUID(),
          session_id: currentSessionId,
          role: 'assistant',
          content: response.response,
          created_at: new Date().toISOString(),
        }
        setMessages((prev) => [...prev, assistantMessage])
      } catch (error) {
        console.error('Failed to send message:', error)
        // Remove the optimistic user message on error
        setMessages((prev) => prev.slice(0, -1))
      } finally {
        setIsLoading(false)
      }
    },
    [currentSessionId]
  )

  return (
    <div className="flex h-[calc(100vh-64px)] bg-background">
      {/* Sidebar */}
      <div className="w-72 shrink-0 border-r border-border">
        <SessionList
          sessions={sessions}
          currentSessionId={currentSessionId ?? undefined}
          onSelectSession={setCurrentSessionId}
          onNewSession={() => setShowNewSessionDialog(true)}
          onDeleteSession={handleDeleteSession}
        />
      </div>

      {/* Main chat area */}
      <div className="flex-1 flex flex-col relative">
        {/* Background pattern */}
        <div className="absolute inset-0 diagonal-lines opacity-30 pointer-events-none" />

        {currentSessionId ? (
          <>
            <ScrollArea className="flex-1 relative">
              <div>
                {messages.map((message, index) => (
                  <ChatMessage key={message.id} message={message} index={index} />
                ))}
                {isLoading && <LoadingIndicator />}
              </div>
            </ScrollArea>
            <ChatInput onSend={handleSendMessage} isLoading={isLoading} />
          </>
        ) : (
          <WelcomeScreen onSendMessage={handleSendMessage} isLoading={isLoading} />
        )}
      </div>

      <SystemPromptDialog
        open={showNewSessionDialog}
        onOpenChange={setShowNewSessionDialog}
        onSubmit={handleNewSession}
      />
    </div>
  )
}

function LoadingIndicator() {
  return (
    <div className="relative p-6 animate-fade-up">
      {/* Left accent line */}
      <div className="absolute left-0 top-0 bottom-0 w-[2px] bg-gradient-to-b from-white/20 via-white/5 to-transparent" />

      <div className="flex gap-4">
        {/* Avatar skeleton */}
        <div className="w-10 h-10 border border-border flex items-center justify-center">
          <div className="w-3 h-3 border border-foreground/30 rotate-45 animate-spin" />
        </div>

        {/* Content skeleton */}
        <div className="flex-1 space-y-3">
          <div className="flex items-center gap-3">
            <span className="text-xs tracking-[0.2em] text-muted-foreground uppercase">
              void
            </span>
            <div className="flex-1 h-[1px] bg-gradient-to-r from-border to-transparent" />
            <span className="text-[10px] text-muted-foreground/30 tracking-wider">
              processing
            </span>
          </div>

          {/* Animated loading bars */}
          <div className="space-y-2">
            {[100, 80, 60].map((width, i) => (
              <div
                key={i}
                className="h-3 bg-secondary animate-pulse"
                style={{
                  width: `${width}%`,
                  animationDelay: `${i * 150}ms`,
                }}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}

function WelcomeScreen({
  onSendMessage,
  isLoading,
}: {
  onSendMessage: (message: string) => void
  isLoading: boolean
}) {
  return (
    <div className="flex-1 flex items-center justify-center p-8 relative">
      {/* Decorative background elements */}
      <div className="absolute inset-0 overflow-hidden pointer-events-none">
        {/* Large geometric shapes */}
        <div className="absolute top-20 left-20 w-40 h-40 border border-border/20 rotate-45 opacity-50" />
        <div className="absolute bottom-32 right-32 w-24 h-24 border border-border/30 rotate-12" />
        <div className="absolute top-1/3 right-1/4 w-16 h-16 border border-border/10 -rotate-12" />

        {/* Gradient orbs */}
        <div className="absolute top-1/4 left-1/3 w-64 h-64 bg-gradient-radial from-white/[0.02] to-transparent rounded-full blur-3xl" />
        <div className="absolute bottom-1/4 right-1/3 w-48 h-48 bg-gradient-radial from-white/[0.01] to-transparent rounded-full blur-2xl" />
      </div>

      <div className="relative max-w-2xl w-full text-center space-y-12">
        {/* Logo */}
        <div className="flex flex-col items-center gap-6 animate-fade-up">
          <div className="relative">
            <div className="w-20 h-20 border-2 border-foreground rotate-45 pulse-glow">
              <div className="absolute inset-2 border border-foreground/30 rotate-0" />
              <div className="absolute inset-4 bg-foreground/5" />
            </div>
            {/* Shadow layers */}
            <div className="absolute inset-0 w-20 h-20 border border-foreground/20 rotate-45 translate-x-1 translate-y-1" />
            <div className="absolute inset-0 w-20 h-20 border border-foreground/10 rotate-45 translate-x-2 translate-y-2" />
          </div>

          <div className="space-y-2">
            <h1 className="font-display text-5xl tracking-[0.3em] gradient-text">
              VOID
            </h1>
            <p className="text-xs tracking-[0.4em] text-muted-foreground uppercase">
              neural interface system
            </p>
          </div>
        </div>

        {/* Divider */}
        <div
          className="flex items-center gap-4 animate-fade-up"
          style={{ animationDelay: '100ms' }}
        >
          <div className="flex-1 h-[1px] bg-gradient-to-r from-transparent via-border to-transparent" />
          <div className="w-2 h-2 border border-foreground/30 rotate-45" />
          <div className="flex-1 h-[1px] bg-gradient-to-r from-transparent via-border to-transparent" />
        </div>

        {/* Description */}
        <div
          className="space-y-4 animate-fade-up"
          style={{ animationDelay: '200ms' }}
        >
          <p className="text-sm text-muted-foreground leading-relaxed max-w-md mx-auto">
            Initialize a conversation with the neural interface.
            <br />
            Your transmissions will be processed and responded to.
          </p>
        </div>

        {/* Input area */}
        <div
          className={cn(
            'animate-fade-up',
            'max-w-xl mx-auto'
          )}
          style={{ animationDelay: '300ms' }}
        >
          <ChatInput onSend={onSendMessage} isLoading={isLoading} />
        </div>

        {/* Footer hints */}
        <div
          className="flex items-center justify-center gap-8 animate-fade-up"
          style={{ animationDelay: '400ms' }}
        >
          {[
            { label: 'sessions', value: '0' },
            { label: 'status', value: 'ready' },
            { label: 'latency', value: '<1ms' },
          ].map((stat) => (
            <div key={stat.label} className="text-center">
              <p className="text-xs text-foreground tracking-wider">
                {stat.value}
              </p>
              <p className="text-[10px] text-muted-foreground/40 tracking-[0.2em] uppercase">
                {stat.label}
              </p>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
