"use client";

import { RulerCarousel, type CarouselItem } from "@/components/ui/ruler-carousel";

const TECH_ITEMS: CarouselItem[] = [
  { id: 1, title: "Ethereum EVM" },
  { id: 2, title: "Monad EVM" },
  { id: 3, title: "Foundry / Solidity" },
  { id: 4, title: "Rust Sidecar" },
  { id: 5, title: "ERC-8354" },
  { id: 6, title: "ERC-8380" },
  { id: 7, title: "ZKOS Labs" },
];

export function TechCarousel() {
  return (
    <section className="border-y border-white/[0.06] overflow-hidden py-24 px-6">
      <p className="font-sans text-sm uppercase tracking-widest text-zinc-500 text-center mb-12">
        Built on
      </p>
      <RulerCarousel originalItems={TECH_ITEMS} />
    </section>
  );
}
