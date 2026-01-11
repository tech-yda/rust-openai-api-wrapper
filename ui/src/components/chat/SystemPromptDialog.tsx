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
import { Textarea } from '@/components/ui/textarea'

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

  const handleSubmit = () => {
    onSubmit(systemPrompt.trim() || undefined)
    setSystemPrompt('')
    onOpenChange(false)
  }

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>New Chat Session</DialogTitle>
          <DialogDescription>
            Optionally set a system prompt to customize the assistant's behavior.
          </DialogDescription>
        </DialogHeader>
        <div className="py-4">
          <Textarea
            value={systemPrompt}
            onChange={(e) => setSystemPrompt(e.target.value)}
            placeholder="You are a helpful assistant..."
            className="min-h-[120px]"
          />
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button onClick={handleSubmit}>Create Session</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
