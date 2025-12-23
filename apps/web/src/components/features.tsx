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
      {/* Section Header */}
      <div className="max-w-3xl mx-auto text-center mb-16">
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
      {/* Icon */}
      <div className="mb-4 inline-flex p-3 bg-primary/10 text-primary border border-primary/20 group-hover:bg-primary/20 group-hover:border-primary/40 transition-all">
        <Icon className="w-5 h-5" />
      </div>

      {/* Content */}
      <h3 className="text-lg font-semibold mb-2 text-foreground group-hover:text-primary transition-colors">
        {feature.title}
      </h3>
      <p className="text-sm text-muted-foreground leading-relaxed">
        {feature.description}
      </p>

      {/* Hover Accent Line */}
      <div className="absolute bottom-0 left-0 right-0 h-[2px] bg-gradient-to-r from-transparent via-primary/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
    </div>
  );
}
