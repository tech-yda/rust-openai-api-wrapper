import { useState, useCallback } from 'react'
import { cn } from '@/lib/utils'

interface ChatInputProps {
  onSend: (message: string) => void
  isLoading?: boolean
  disabled?: boolean
}

export function ChatInput({ onSend, isLoading, disabled }: ChatInputProps) {
  const [message, setMessage] = useState('')
  const [isFocused, setIsFocused] = useState(false)

  const handleSubmit = useCallback(() => {
    const trimmed = message.trim()
    if (trimmed && !isLoading && !disabled) {
      onSend(trimmed)
      setMessage('')
    }
  }, [message, isLoading, disabled, onSend])

  const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault()
      handleSubmit()
    }
  }

  return (
    <div className="relative p-4 bg-background border-t border-border">
      {/* Top gradient line */}
      <div className="absolute top-0 left-0 right-0 h-[1px] bg-gradient-to-r from-transparent via-white/10 to-transparent" />

      {/* Input container */}
      <div
        className={cn(
          'relative group',
          'transition-all duration-300'
        )}
      >
        {/* Animated border */}
        <div
          className={cn(
            'absolute inset-0 border transition-colors duration-300',
            isFocused ? 'border-foreground/50' : 'border-border'
          )}
        />

        {/* Gradient border on focus */}
        {isFocused && (
          <div className="absolute inset-0 gradient-border" />
        )}

        {/* Corner decorations */}
        <div className="absolute top-0 left-0 w-3 h-3 border-l-2 border-t-2 border-foreground/30 -translate-x-[1px] -translate-y-[1px]" />
        <div className="absolute top-0 right-0 w-3 h-3 border-r-2 border-t-2 border-foreground/30 translate-x-[1px] -translate-y-[1px]" />
        <div className="absolute bottom-0 left-0 w-3 h-3 border-l-2 border-b-2 border-foreground/30 -translate-x-[1px] translate-y-[1px]" />
        <div className="absolute bottom-0 right-0 w-3 h-3 border-r-2 border-b-2 border-foreground/30 translate-x-[1px] translate-y-[1px]" />

        <div className="relative flex items-end gap-3 p-3">
          {/* Textarea */}
          <div className="flex-1 relative">
            <textarea
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              onKeyDown={handleKeyDown}
              onFocus={() => setIsFocused(true)}
              onBlur={() => setIsFocused(false)}
              placeholder="transmit your message..."
              disabled={isLoading || disabled}
              className={cn(
                'w-full min-h-[60px] max-h-[200px] resize-none',
                'bg-transparent text-sm text-foreground',
                'placeholder:text-muted-foreground/50 placeholder:tracking-wider',
                'focus:outline-none',
                'disabled:opacity-50 disabled:cursor-not-allowed'
              )}
              rows={2}
            />

            {/* Character count */}
            {message.length > 0 && (
              <span className="absolute bottom-0 right-0 text-[10px] text-muted-foreground/40 tracking-wider">
                {message.length}
              </span>
            )}
          </div>

          {/* Send button */}
          <button
            onClick={handleSubmit}
            disabled={!message.trim() || isLoading || disabled}
            className={cn(
              'relative shrink-0',
              'w-12 h-12',
              'border border-border',
              'bg-transparent',
              'transition-all duration-300',
              'disabled:opacity-30 disabled:cursor-not-allowed',
              'group/btn',
              message.trim() && !isLoading && !disabled && [
                'hover:bg-foreground hover:border-foreground',
                'hover:text-background',
                'active:scale-95'
              ]
            )}
          >
            {/* Button content */}
            {isLoading ? (
              <div className="flex items-center justify-center">
                <div className="w-4 h-4 border border-foreground/50 animate-spin" />
              </div>
            ) : (
              <div className="flex items-center justify-center">
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 16 16"
                  fill="none"
                  className={cn(
                    'transition-transform duration-300',
                    message.trim() && 'group-hover/btn:-translate-y-[2px] group-hover/btn:translate-x-[2px]'
                  )}
                >
                  <path
                    d="M1 8L14 1L7 14L6 9L1 8Z"
                    stroke="currentColor"
                    strokeWidth="1.5"
                    strokeLinejoin="bevel"
                  />
                </svg>
              </div>
            )}

            {/* Button corner accent */}
            <div className="absolute top-0 right-0 w-2 h-2 bg-foreground/20 opacity-0 group-hover/btn:opacity-100 transition-opacity" />
          </button>
        </div>
      </div>

      {/* Hint text */}
      <div className="flex items-center justify-between mt-2 px-1">
        <span className="text-[10px] text-muted-foreground/40 tracking-wider">
          shift + enter for new line
        </span>
        <div className="flex items-center gap-1">
          {[...Array(3)].map((_, i) => (
            <div
              key={i}
              className={cn(
                'w-1 h-1',
                isLoading
                  ? 'bg-foreground/40 animate-pulse'
                  : 'bg-foreground/10'
              )}
              style={{
                animationDelay: `${i * 200}ms`,
              }}
            />
          ))}
        </div>
      </div>
    </div>
  )
}
