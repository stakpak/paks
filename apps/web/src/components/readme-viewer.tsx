import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
// TODO: Switch back to ESM import once react-syntax-highlighter fixes their broken ESM exports
// See: https://github.com/react-syntax-highlighter/react-syntax-highlighter/issues
// Original: import { oneDark } from "react-syntax-highlighter/dist/esm/styles/prism";
import { oneDark } from "react-syntax-highlighter/dist/cjs/styles/prism";
import { FileText, Loader2 } from "lucide-react";

interface SkillViewerProps {
  content: string | null;
  isLoading?: boolean;
}

export function SkillViewer({ content, isLoading }: SkillViewerProps) {
  if (isLoading) {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12">
          <Loader2 className="w-6 h-6 text-primary animate-spin mb-3" />
          <p className="text-sm text-muted-foreground">Loading skill.md...</p>
        </div>
      </div>
    );
  }

  if (!content) {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <FileText className="w-10 h-10 text-muted-foreground mb-4" />
          <h3 className="text-lg font-medium text-foreground mb-2">No SKILL.md</h3>
          <p className="text-sm text-muted-foreground max-w-sm">
            This package doesn't have a SKILL.md file yet.
          </p>
        </div>
      </div>
    );
  }

  return (
    <div className="glass border border-border/30 p-6 sm:p-8">
      <article className="prose prose-invert prose-sm sm:prose-base max-w-none
        prose-headings:text-foreground prose-headings:font-bold prose-headings:border-b prose-headings:border-border/30 prose-headings:pb-2 prose-headings:mb-4
        prose-h1:text-2xl prose-h2:text-xl prose-h3:text-lg
        prose-p:text-muted-foreground prose-p:leading-relaxed
        prose-a:text-primary prose-a:no-underline hover:prose-a:underline
        prose-strong:text-foreground
        prose-code:text-primary prose-code:bg-primary/10 prose-code:px-1.5 prose-code:py-0.5 prose-code:text-sm prose-code:before:content-none prose-code:after:content-none
        prose-pre:bg-transparent prose-pre:p-0
        prose-ul:text-muted-foreground prose-ol:text-muted-foreground
        prose-li:marker:text-primary/60
        prose-blockquote:border-l-primary prose-blockquote:bg-primary/5 prose-blockquote:pl-4 prose-blockquote:py-2 prose-blockquote:text-muted-foreground
        prose-hr:border-border/30
        prose-table:border prose-table:border-border/30
        prose-th:bg-muted/30 prose-th:px-3 prose-th:py-2 prose-th:text-foreground
        prose-td:px-3 prose-td:py-2 prose-td:border prose-td:border-border/30
      ">
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          components={{
            code({ className, children, ...props }) {
              const match = /language-(\w+)/.exec(className || "");
              const codeString = String(children).replace(/\n$/, "");
              
              // Check if it's an inline code block (no language specified and short content)
              const isInline = !match && !codeString.includes("\n");
              
              if (isInline) {
                return (
                  <code className={className} {...props}>
                    {children}
                  </code>
                );
              }
              
              return (
                <div className="relative group not-prose my-4">
                  <div className="absolute top-2 right-2 px-2 py-0.5 text-[10px] text-muted-foreground bg-muted/50 opacity-0 group-hover:opacity-100 transition-opacity">
                    {match?.[1] || "code"}
                  </div>
                  <SyntaxHighlighter
                    style={oneDark}
                    language={match?.[1] || "text"}
                    PreTag="div"
                    customStyle={{
                      margin: 0,
                      padding: "1rem",
                      background: "oklch(0.15 0.01 250 / 80%)",
                      border: "1px solid oklch(0.30 0.04 200 / 30%)",
                      fontSize: "0.875rem",
                    }}
                  >
                    {codeString}
                  </SyntaxHighlighter>
                </div>
              );
            },
            img({ src, alt }) {
              return (
                <img 
                  src={src} 
                  alt={alt || ""} 
                  className="max-w-full h-auto border border-border/30" 
                  loading="lazy"
                />
              );
            },
          }}
        >
          {content}
        </ReactMarkdown>
      </article>
    </div>
  );
}
