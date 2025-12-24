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

// Remove YAML frontmatter from markdown content
function stripFrontmatter(content: string): string {
  // Match frontmatter block: starts with ---, ends with ---
  const frontmatterRegex = /^---\s*\n[\s\S]*?\n---\s*\n?/;
  return content.replace(frontmatterRegex, '').trim();
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

  // Strip frontmatter before rendering
  const cleanContent = stripFrontmatter(content);

  return (
    <div className="glass border border-border/30 p-6 sm:p-8">
      <article className="
        prose prose-invert max-w-none
        prose-headings:text-foreground prose-headings:font-bold prose-headings:mt-8 prose-headings:mb-4
        prose-h1:text-3xl prose-h1:border-b prose-h1:border-border/40 prose-h1:pb-3 prose-h1:mt-0
        prose-h2:text-2xl prose-h2:border-b prose-h2:border-border/30 prose-h2:pb-2
        prose-h3:text-xl prose-h3:font-semibold
        prose-h4:text-lg
        prose-p:text-foreground/90 prose-p:leading-7 prose-p:my-4
        prose-a:text-primary prose-a:underline prose-a:underline-offset-2 hover:prose-a:text-primary/80
        prose-strong:text-foreground prose-strong:font-semibold
        prose-code:text-primary prose-code:bg-primary/10 prose-code:px-1.5 prose-code:py-0.5 prose-code:text-sm prose-code:rounded prose-code:before:content-none prose-code:after:content-none
        prose-pre:bg-transparent prose-pre:p-0 prose-pre:my-4
        prose-ul:my-4 prose-ul:pl-6 prose-ul:list-disc
        prose-ol:my-4 prose-ol:pl-6 prose-ol:list-decimal
        prose-li:text-foreground/90 prose-li:my-1 prose-li:leading-7
        prose-li:marker:text-muted-foreground
        prose-blockquote:border-l-4 prose-blockquote:border-primary/50 prose-blockquote:bg-muted/20 prose-blockquote:pl-4 prose-blockquote:py-1 prose-blockquote:my-4 prose-blockquote:text-foreground/80 prose-blockquote:not-italic
        prose-hr:border-border/40 prose-hr:my-8
        prose-table:border prose-table:border-border/30 prose-table:my-4
        prose-th:bg-muted/40 prose-th:px-4 prose-th:py-2 prose-th:text-foreground prose-th:font-semibold prose-th:text-left
        prose-td:px-4 prose-td:py-2 prose-td:border prose-td:border-border/30
        prose-img:rounded prose-img:my-4
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
          {cleanContent}
        </ReactMarkdown>
      </article>
    </div>
  );
}
