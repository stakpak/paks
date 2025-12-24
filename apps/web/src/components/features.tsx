import { Package, Download, Search, Terminal, Plug, GitBranch } from "lucide-react";

const features = [
  {
    icon: Package,
    title: "Create Skills",
    description: "Package your team's best practices, coding standards, and domain knowledge as shareable skills.",
  },
  {
    icon: Download,
    title: "Install Anywhere",
    description: "Works with Claude Code, Cursor, VS Code, GitHub Copilot, Goose, and more AI agents.",
  },
  {
    icon: Search,
    title: "Discover & Share",
    description: "Browse community skills or publish your own to help others build better with AI.",
  },
  {
    icon: Terminal,
    title: "CLI-First",
    description: "Powerful command-line interface for creating, validating, and managing skills effortlessly.",
  },
  {
    icon: Plug,
    title: "Seamless Integration",
    description: "Skills follow the Agent Skills spec, ensuring compatibility across the AI ecosystem.",
  },
  {
    icon: GitBranch,
    title: "Version Control",
    description: "Semantic versioning for skills. Track updates, manage dependencies, and iterate safely.",
  },
];

export function Features() {
  return (
    <section className="relative py-24 px-4 sm:px-6 lg:px-8">
      {/* Background Accent - Orange Glow */}
      <div className="absolute inset-0 bg-gradient-to-b from-transparent via-orange-500/[0.02] to-transparent pointer-events-none" />

      {/* Section Header */}
      <div className="max-w-3xl mx-auto text-center mb-16 relative">
        <h2 className="text-3xl sm:text-4xl font-bold mb-4">
          <span className="gradient-text">Everything you need</span>
        </h2>
        <p className="text-lg text-muted-foreground">
          A complete toolkit for managing AI agent skills across your entire workflow.
        </p>
      </div>

      {/* Features Grid */}
      <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {features.map((feature, index) => (
          <FeatureCard key={feature.title} feature={feature} index={index} />
        ))}
      </div>
    </section>
  );
}

function FeatureCard({ feature, index }: { feature: typeof features[0]; index: number }) {
  const Icon = feature.icon;
  
  return (
    <div 
      className="group relative p-6 glass hover-lift hover-glow transition-all duration-300"
      style={{ animationDelay: `${index * 100}ms` }}
    >
      {/* Icon - Orange Glow */}
      <div 
        className="mb-4 inline-flex p-3 bg-orange-500/10 text-orange-400 border border-orange-500/20 group-hover:bg-orange-500/20 group-hover:border-orange-500/40 transition-all"
        style={{ boxShadow: '0 0 12px oklch(0.7 0.15 50 / 15%)' }}
      >
        <Icon className="w-5 h-5" />
      </div>

      {/* Content */}
      <h3 className="text-lg font-semibold mb-2 text-foreground group-hover:text-orange-400 transition-colors">
        {feature.title}
      </h3>
      <p className="text-sm text-muted-foreground leading-relaxed">
        {feature.description}
      </p>

      {/* Hover Accent Line - Orange */}
      <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-gradient-to-r from-transparent via-orange-500/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
    </div>
  );
}
