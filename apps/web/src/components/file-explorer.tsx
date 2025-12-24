import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { PaksClient } from "@paks/api";
import type { PakContent, ContentItem } from "@paks/api";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { oneDark } from "react-syntax-highlighter/dist/cjs/styles/prism";
import { Folder, FileText, Loader2, ChevronLeft } from "lucide-react";

interface FileExplorerProps {
  content?: PakContent;
  isLoading?: boolean;
  pakUri: string;
  skillContent?: string | null;
}

export function FileExplorer({ content, isLoading, pakUri, skillContent }: FileExplorerProps) {
  // Current path in the file browser (empty = root, or path segments)
  const [currentPath, setCurrentPath] = useState<string>("");
  // Currently viewing file content (null = showing directory listing)
  const [viewingFile, setViewingFile] = useState<string | null>(null);

  const client = new PaksClient();

  // Fetch directory content for current path
  const { data: dirContent, isLoading: dirLoading } = useQuery({
    queryKey: ["dirContent", pakUri, currentPath],
    queryFn: () => client.getPakContent(currentPath ? `${pakUri}/${currentPath}` : pakUri),
    enabled: !viewingFile,
    staleTime: 60000,
  });

  // Fetch file content when viewing a file
  const { data: fileContent, isLoading: fileLoading } = useQuery({
    queryKey: ["fileContent", pakUri, viewingFile],
    queryFn: () => client.getPakContent(`${pakUri}/${viewingFile}`),
    enabled: !!viewingFile,
    staleTime: 60000,
  });

  if (isLoading) {
    return (
      <div className="glass border border-border/30 p-8">
        <div className="flex flex-col items-center justify-center py-12">
          <Loader2 className="w-6 h-6 text-primary animate-spin mb-3" />
          <p className="text-sm text-muted-foreground">Loading files...</p>
        </div>
      </div>
    );
  }

  // Helper to extract filename from URI
  const getFilenameFromUri = (uri: string) => uri.split('/').pop() || uri;

  // Normalize API response items
  const normalizeItem = (item: ContentItem): ContentItem => ({
    ...item,
    name: item.name || getFilenameFromUri(item.uri),
    type: item.type || ((item as unknown as Record<string, unknown>).content_type as "file" | "dir") || "file"
  });

  // Get file items from content - either from API or fallback to skillContent
  const getFileItems = (): ContentItem[] => {
    // If viewing file, return empty (we show file content instead)
    if (viewingFile) return [];
    
    // Use dirContent if available
    if (dirContent?.content?.type === "Directory" && dirContent.content.items.length > 0) {
      return dirContent.content.items.map(normalizeItem);
    }
    
    // Use initial content prop
    if (content?.type === "Directory" && content.items.length > 0) {
      return content.items.map(normalizeItem);
    }
    
    // Fallback: show SKILL.md if we have it
    if (skillContent) {
      return [{
        name: "SKILL.md",
        type: "file" as const,
        uri: `${pakUri}/SKILL.md`,
        content: skillContent
      }];
    }
    
    return [];
  };

  const fileItems = getFileItems();

  // Handle clicking on a file or directory
  const handleItemClick = (item: ContentItem) => {
    if (item.type === "dir") {
      // Navigate into directory
      const newPath = currentPath ? `${currentPath}/${item.name}` : item.name;
      setCurrentPath(newPath);
    } else {
      // View file content
      const filePath = currentPath ? `${currentPath}/${item.name}` : item.name;
      setViewingFile(filePath);
    }
  };

  // Go back (up one directory or exit file view)
  const handleBack = () => {
    if (viewingFile) {
      setViewingFile(null);
    } else if (currentPath) {
      const parts = currentPath.split('/');
      parts.pop();
      setCurrentPath(parts.join('/'));
    }
  };

  // Build path display 
  const getPathDisplay = () => {
    const parts = [pakUri.split('/').pop() || pakUri]; // Package name
    if (currentPath) {
      parts.push(...currentPath.split('/'));
    }
    if (viewingFile) {
      parts.push(viewingFile.split('/').pop() || viewingFile);
    }
    return '/ ' + parts.join(' / ') + ' /';
  };

  // File view mode
  if (viewingFile) {
    const fileName = viewingFile.split('/').pop() || viewingFile;
    const fileData = fileContent?.content?.type === "File" ? fileContent.content.content : null;
    const lineCount = fileData?.split('\n').length || 0;
    
    return (
      <div className="glass border border-border/30">
        {/* Path header */}
        <div className="px-4 py-3 border-b border-border/30">
          <span className="text-sm text-muted-foreground font-mono">{getPathDisplay()}</span>
        </div>
        
        {/* Back button and file info */}
        <div className="flex items-center justify-between px-4 py-2 border-b border-border/30 bg-muted/10">
          <button
            onClick={handleBack}
            className="flex items-center gap-1 text-sm text-muted-foreground hover:text-foreground transition-colors"
          >
            <ChevronLeft className="w-4 h-4" />
            Back
          </button>
          <div className="flex items-center gap-4 text-xs text-muted-foreground">
            <span>{lineCount} LOC</span>
          </div>
        </div>
        
        {/* File content */}
        <div className="overflow-auto max-h-[600px] scrollbar-thin">
          {fileLoading ? (
            <div className="flex items-center justify-center py-12">
              <Loader2 className="w-5 h-5 text-primary animate-spin" />
            </div>
          ) : fileData ? (
            <SyntaxHighlighter
              style={oneDark}
              language={getLanguageFromFilename(fileName)}
              showLineNumbers
              customStyle={{
                margin: 0,
                padding: "1rem",
                background: "transparent",
                fontSize: "0.8rem",
              }}
              lineNumberStyle={{
                minWidth: "3em",
                paddingRight: "1em",
                color: "oklch(0.5 0.02 200)",
                userSelect: "none",
              }}
            >
              {fileData}
            </SyntaxHighlighter>
          ) : (
            <div className="p-4 text-sm text-muted-foreground">
              Unable to load file content
            </div>
          )}
        </div>
      </div>
    );
  }

  // Directory listing mode
  const showBackButton = currentPath !== "";

  return (
    <div className="glass border border-border/30">
      {/* Path header */}
      <div className="px-4 py-3 border-b border-border/30">
        <span className="text-sm text-muted-foreground font-mono">{getPathDisplay()}</span>
      </div>

      {/* File/Directory listing */}
      <div className="divide-y divide-border/20">
        {/* Back button (../) */}
        {showBackButton && (
          <button
            onClick={handleBack}
            className="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-muted/20 transition-colors"
          >
            <Folder className="w-4 h-4 text-primary/70" />
            <span className="text-sm text-muted-foreground">..</span>
          </button>
        )}

        {dirLoading ? (
          <div className="flex items-center justify-center py-12">
            <Loader2 className="w-5 h-5 text-primary animate-spin" />
          </div>
        ) : fileItems.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 text-center">
            <Folder className="w-10 h-10 text-muted-foreground mb-4" />
            <h3 className="text-lg font-medium text-foreground mb-2">No Files</h3>
            <p className="text-sm text-muted-foreground">
              This directory is empty.
            </p>
          </div>
        ) : (
          fileItems.map((item) => (
            <button
              key={item.uri}
              onClick={() => handleItemClick(item)}
              className="w-full flex items-center gap-3 px-4 py-3 text-left hover:bg-muted/20 transition-colors group"
            >
              {item.type === "dir" ? (
                <Folder className="w-4 h-4 text-primary/70 flex-shrink-0" />
              ) : (
                <FileText className="w-4 h-4 text-muted-foreground flex-shrink-0" />
              )}
              <span className="text-sm text-foreground group-hover:text-primary transition-colors flex-1">
                {item.name}{item.type === "dir" ? "/" : ""}
              </span>
              <span className="text-xs text-muted-foreground">
                {item.type === "dir" ? "folder" : getMimeType(item.name)}
              </span>
              {item.size && (
                <span className="text-xs text-muted-foreground">
                  {formatFileSize(item.size)}
                </span>
              )}
            </button>
          ))
        )}
      </div>
    </div>
  );
}

// Get mime type from filename
function getMimeType(filename: string): string {
  if (!filename) return "text/plain";
  const ext = filename.split(".").pop()?.toLowerCase();
  const mimeMap: Record<string, string> = {
    md: "text/markdown",
    js: "application/javascript",
    ts: "application/typescript",
    tsx: "application/typescript",
    jsx: "application/javascript",
    json: "application/json",
    yaml: "text/yaml",
    yml: "text/yaml",
    toml: "text/toml",
    py: "text/python",
    rs: "text/rust",
    go: "text/go",
    html: "text/html",
    css: "text/css",
    txt: "text/plain",
  };
  return mimeMap[ext || ""] || "text/plain";
}

// Get syntax highlighting language from filename
function getLanguageFromFilename(filename: string): string {
  if (!filename) return "text";
  const ext = filename.split(".").pop()?.toLowerCase();
  const langMap: Record<string, string> = {
    js: "javascript",
    jsx: "jsx",
    ts: "typescript",
    tsx: "tsx",
    py: "python",
    rs: "rust",
    go: "go",
    rb: "ruby",
    java: "java",
    c: "c",
    cpp: "cpp",
    h: "c",
    hpp: "cpp",
    md: "markdown",
    json: "json",
    yaml: "yaml",
    yml: "yaml",
    toml: "toml",
    sh: "bash",
    bash: "bash",
    zsh: "bash",
    css: "css",
    scss: "scss",
    html: "html",
    xml: "xml",
    sql: "sql",
  };
  return langMap[ext || ""] || "text";
}

// Format file size
function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(2)} kB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}
