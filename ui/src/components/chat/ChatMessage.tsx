import { cn } from '@/lib/utils'
import type { Message } from '@/lib/api'

interface ChatMessageProps {
  message: Message
  index?: number
}

export function ChatMessage({ message, index = 0 }: ChatMessageProps) {
  const isUser = message.role === 'user'

  return (
    <div
      className={cn(
        'relative group',
        'animate-fade-up'
      )}
      style={{ animationDelay: `${index * 50}ms` }}
    >
      {/* Message container */}
      <div
        className={cn(
          'relative flex gap-4 p-6',
          isUser
            ? 'bg-secondary/30'
            : 'bg-background'
        )}
      >
        {/* Left accent line */}
        <div
          className={cn(
            'absolute left-0 top-0 bottom-0 w-[2px]',
            isUser
              ? 'bg-gradient-to-b from-white/40 via-white/20 to-transparent'
              : 'bg-gradient-to-b from-white/20 via-white/5 to-transparent'
          )}
        />

        {/* Avatar */}
        <div className="relative shrink-0">
          <div
            className={cn(
              'w-10 h-10 flex items-center justify-center',
              'border transition-all duration-300',
              isUser
                ? 'border-foreground bg-foreground text-background'
                : 'border-border bg-transparent text-foreground'
            )}
          >
            {isUser ? (
              <span className="font-display text-lg">U</span>
            ) : (
              <div className="relative">
                <div className="w-3 h-3 border border-foreground rotate-45" />
                <div className="absolute inset-0 w-3 h-3 border border-foreground/30 rotate-45 translate-x-[1px] translate-y-[1px]" />
              </div>
            )}
          </div>

          {/* Decorative corner */}
          {!isUser && (
            <div className="absolute -bottom-1 -right-1 w-2 h-2 border-r border-b border-border" />
          )}
        </div>

        {/* Content */}
        <div className="flex-1 min-w-0 space-y-2">
          {/* Header */}
          <div className="flex items-center gap-3">
            <span
              className={cn(
                'text-xs font-medium tracking-[0.2em] uppercase',
                isUser ? 'text-foreground' : 'text-muted-foreground'
              )}
            >
              {isUser ? 'you' : 'void'}
            </span>
            <div className="flex-1 h-[1px] bg-gradient-to-r from-border to-transparent" />
            <span className="text-[10px] text-muted-foreground/50 tracking-wider">
              {new Date(message.created_at).toLocaleTimeString('en-US', {
                hour: '2-digit',
                minute: '2-digit',
                hour12: false,
              })}
            </span>
          </div>

          {/* Message content */}
          <div
            className={cn(
              'text-sm leading-relaxed',
              isUser ? 'text-foreground' : 'text-foreground/90'
            )}
          >
            <p className="whitespace-pre-wrap">{message.content}</p>
          </div>
        </div>

        {/* Hover decoration */}
        <div
          className={cn(
            'absolute right-4 top-1/2 -translate-y-1/2',
            'opacity-0 group-hover:opacity-100 transition-opacity duration-300',
            'flex items-center gap-1'
          )}
        >
          {[...Array(3)].map((_, i) => (
            <div
              key={i}
              className="w-1 h-1 bg-foreground/20"
              style={{
                animationDelay: `${i * 100}ms`,
              }}
            />
          ))}
        </div>
      </div>

      {/* Bottom separator */}
      <div className="h-[1px] bg-gradient-to-r from-border/50 via-border to-border/50" />
    </div>
  )
}
