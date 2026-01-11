import { useState } from 'react'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import { cn } from '@/lib/utils'

interface SystemPromptDialogProps {
  open: boolean
  onOpenChange: (open: boolean) => void
  onSubmit: (systemPrompt: string | undefined) => void
}

export function SystemPromptDialog({
  open,
  onOpenChange,
  onSubmit,
}: SystemPromptDialogProps) {
  const [systemPrompt, setSystemPrompt] = useState('')
  const [isFocused, setIsFocused] = useState(false)

  const handleSubmit = () => {
    onSubmit(systemPrompt.trim() || undefined)
    setSystemPrompt('')
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>INITIALIZE SESSION</DialogTitle>
          <DialogDescription>
            configure the neural interface behavior with a system directive
          </DialogDescription>
        </DialogHeader>

        <div className="py-4">
          {/* Textarea container */}
          <div className="relative">
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
            <div className="absolute top-0 left-0 w-2 h-2 border-l border-t border-foreground/30 -translate-x-[1px] -translate-y-[1px]" />
            <div className="absolute top-0 right-0 w-2 h-2 border-r border-t border-foreground/30 translate-x-[1px] -translate-y-[1px]" />
            <div className="absolute bottom-0 left-0 w-2 h-2 border-l border-b border-foreground/30 -translate-x-[1px] translate-y-[1px]" />
            <div className="absolute bottom-0 right-0 w-2 h-2 border-r border-b border-foreground/30 translate-x-[1px] translate-y-[1px]" />

            <textarea
              value={systemPrompt}
              onChange={(e) => setSystemPrompt(e.target.value)}
              onFocus={() => setIsFocused(true)}
              onBlur={() => setIsFocused(false)}
              placeholder="you are a helpful assistant..."
              className={cn(
                'relative w-full min-h-[140px] resize-none',
                'bg-transparent text-sm text-foreground',
                'p-4',
                'placeholder:text-muted-foreground/40 placeholder:tracking-wider',
                'focus:outline-none'
              )}
            />

            {/* Character count */}
            {systemPrompt.length > 0 && (
              <span className="absolute bottom-2 right-3 text-[10px] text-muted-foreground/40 tracking-wider">
                {systemPrompt.length}
              </span>
            )}
          </div>

          {/* Helper text */}
          <p className="mt-2 text-[10px] text-muted-foreground/50 tracking-wider">
            leave empty for default behavior
          </p>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            abort
          </Button>
          <Button onClick={handleSubmit}>
            initialize
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
