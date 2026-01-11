import { cn } from '@/lib/utils'
import type { Session } from '@/lib/api'
import { ScrollArea } from '@/components/ui/scroll-area'

interface SessionListProps {
  sessions: Session[]
  currentSessionId?: string
  onSelectSession: (id: string) => void
  onNewSession: () => void
  onDeleteSession: (id: string) => void
}

export function SessionList({
  sessions,
  currentSessionId,
  onSelectSession,
  onNewSession,
  onDeleteSession,
}: SessionListProps) {
  return (
    <div className="relative flex h-full flex-col bg-background">
      {/* Decorative side line */}
      <div className="absolute right-0 top-0 bottom-0 w-[1px] bg-gradient-to-b from-white/20 via-white/5 to-transparent" />

      {/* Header */}
      <div className="relative p-4 border-b border-border">
        <div className="flex items-center gap-2 mb-3">
          <div className="w-2 h-2 border border-foreground rotate-45" />
          <span className="text-[10px] tracking-[0.3em] text-muted-foreground uppercase">
            sessions
          </span>
        </div>

        {/* New chat button */}
        <button
          onClick={onNewSession}
          className={cn(
            'relative w-full group',
            'p-3 border border-border',
            'bg-transparent hover:bg-foreground',
            'text-foreground hover:text-background',
            'transition-all duration-300',
            'active:scale-[0.98]'
          )}
        >
          {/* Corner decorations */}
          <div className="absolute top-0 left-0 w-2 h-2 border-l border-t border-foreground/50 -translate-x-[1px] -translate-y-[1px] opacity-0 group-hover:opacity-100 transition-opacity" />
          <div className="absolute bottom-0 right-0 w-2 h-2 border-r border-b border-foreground/50 translate-x-[1px] translate-y-[1px] opacity-0 group-hover:opacity-100 transition-opacity" />

          <div className="flex items-center justify-center gap-2">
            <span className="text-lg leading-none">+</span>
            <span className="text-xs tracking-[0.15em] uppercase">
              new session
            </span>
          </div>
        </button>
      </div>

      {/* Sessions list */}
      <ScrollArea className="flex-1">
        <div className="p-2 space-y-1">
          {sessions.length === 0 ? (
            <div className="py-8 text-center">
              <div className="inline-flex flex-col items-center gap-3">
                <div className="relative">
                  <div className="w-8 h-8 border border-border rotate-45" />
                  <div className="absolute inset-0 w-8 h-8 border border-border/30 rotate-45 translate-x-1 translate-y-1" />
                </div>
                <span className="text-xs text-muted-foreground tracking-wider">
                  no sessions yet
                </span>
              </div>
            </div>
          ) : (
            sessions.map((session, index) => (
              <div
                key={session.id}
                className={cn(
                  'relative group cursor-pointer',
                  'animate-fade-up'
                )}
                style={{ animationDelay: `${index * 30}ms` }}
                onClick={() => onSelectSession(session.id)}
              >
                <div
                  className={cn(
                    'relative p-3 border transition-all duration-200',
                    currentSessionId === session.id
                      ? 'border-foreground/50 bg-secondary/50'
                      : 'border-transparent hover:border-border hover:bg-secondary/20'
                  )}
                >
                  {/* Active indicator */}
                  {currentSessionId === session.id && (
                    <div className="absolute left-0 top-2 bottom-2 w-[2px] bg-gradient-to-b from-foreground via-foreground/50 to-transparent" />
                  )}

                  <div className="flex items-start gap-3">
                    {/* Icon */}
                    <div
                      className={cn(
                        'w-6 h-6 flex items-center justify-center shrink-0',
                        'border transition-colors',
                        currentSessionId === session.id
                          ? 'border-foreground/50'
                          : 'border-border'
                      )}
                    >
                      <div className="w-2 h-2 border border-current rotate-45" />
                    </div>

                    {/* Content */}
                    <div className="flex-1 min-w-0">
                      <p className="text-xs text-foreground truncate">
                        {session.system_prompt
                          ? session.system_prompt.slice(0, 35) + (session.system_prompt.length > 35 ? '...' : '')
                          : 'New Session'}
                      </p>
                      <p className="text-[10px] text-muted-foreground/50 mt-1 tracking-wider">
                        {new Date(session.created_at).toLocaleDateString('en-US', {
                          month: 'short',
                          day: 'numeric',
                          hour: '2-digit',
                          minute: '2-digit',
                        })}
                      </p>
                    </div>

                    {/* Delete button */}
                    <button
                      className={cn(
                        'w-6 h-6 flex items-center justify-center',
                        'opacity-0 group-hover:opacity-100',
                        'text-muted-foreground hover:text-destructive',
                        'transition-all duration-200'
                      )}
                      onClick={(e) => {
                        e.stopPropagation()
                        onDeleteSession(session.id)
                      }}
                    >
                      <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
                        <path
                          d="M2 2L10 10M10 2L2 10"
                          stroke="currentColor"
                          strokeWidth="1.5"
                        />
                      </svg>
                    </button>
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </ScrollArea>

      {/* Footer decoration */}
      <div className="p-4 border-t border-border">
        <div className="flex items-center justify-between">
          <span className="text-[10px] text-muted-foreground/40 tracking-wider">
            {sessions.length} session{sessions.length !== 1 ? 's' : ''}
          </span>
          <div className="flex items-center gap-1">
            {[...Array(4)].map((_, i) => (
              <div
                key={i}
                className="w-1 bg-foreground/10"
                style={{ height: `${4 + i * 2}px` }}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
