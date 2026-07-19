import { useEffect } from 'react';
import Lenis from 'lenis';

/* ─────────────────────────────────────────────────────────────────────────────
 * Smooth scroll, and the scroll position as a CSS custom property.
 *
 * Lenis drives the scroll; every frame it writes the current offset to `--sy`
 * (unitless pixels) on the document element. Anything that wants to move with
 * the scroll reads that variable in CSS — no per-element observers, no React
 * re-renders, and critically no entrance animation gating whether content
 * exists. Elements that parallax are already on screen and already readable;
 * only their offset changes.
 *
 * Under `prefers-reduced-motion` Lenis is never constructed, native scrolling is
 * left alone, and `--sy` stays at 0 so every transform resolves to none.
 * ──────────────────────────────────────────────────────────────────────────── */

export function SmoothScroll({ children }: { children: React.ReactNode }) {
  // The nav's scrolled state rides a native listener so it still works when
  // smooth scrolling is disabled for reduced motion.
  useEffect(() => {
    const onNativeScroll = () => {
      document.body.classList.toggle('scrolled', window.scrollY > 8);
    };
    onNativeScroll();
    window.addEventListener('scroll', onNativeScroll, { passive: true });
    return () => window.removeEventListener('scroll', onNativeScroll);
  }, []);

  useEffect(() => {
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    if (reduced) return;

    const lenis = new Lenis({
      duration: 1.05,
      // Gentle exponential ease-out: quick to respond, unhurried to settle.
      easing: (t: number) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
      smoothWheel: true,
      touchMultiplier: 1.6,
    });

    const root = document.documentElement;
    let frame = 0;

    const onScroll = ({ scroll }: { scroll: number }) => {
      root.style.setProperty('--sy', String(Math.round(scroll)));
    };
    lenis.on('scroll', onScroll);

    const raf = (time: number) => {
      lenis.raf(time);
      frame = requestAnimationFrame(raf);
    };
    frame = requestAnimationFrame(raf);

    return () => {
      cancelAnimationFrame(frame);
      lenis.destroy();
      root.style.removeProperty('--sy');
    };
  }, []);

  return <>{children}</>;
}
