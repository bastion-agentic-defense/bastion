/* ─────────────────────────────────────────────────────────────────────────────
 * The Bastion mark.
 *
 * Original geometry, built from the shape already doing work inside the
 * product: the bracket that marks a verdict in the policy ledger. Mirrored, the
 * two brackets close into a gate with a gap at its centre — an aperture that
 * something must pass through to get out. That is literally what the runtime
 * is, so the mark carries meaning rather than decorating.
 *
 * Drawn on a 24-unit grid with a consistent 3-unit stroke and round joins, so
 * it holds at favicon size and at the display size used as the hero artifact.
 * ──────────────────────────────────────────────────────────────────────────── */

interface Props {
  size?: number;
  className?: string;
  /** Any CSS colour. Defaults to the current text colour. */
  color?: string;
  title?: string;
}

export function BastionMark({ size = 24, className, color = 'currentColor', title }: Props) {
  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 24 24"
      fill="none"
      className={className}
      role={title ? 'img' : undefined}
      aria-label={title}
      aria-hidden={title ? undefined : true}
    >
      {/* Left bracket: cuts in on a diagonal, runs straight, kicks back out. */}
      <path
        d="M9.4 2.5 L3.5 6.4 L3.5 17.6 L9.4 21.5"
        stroke={color}
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      {/* Right bracket: the mirror. The gap between them is the aperture. */}
      <path
        d="M14.6 2.5 L20.5 6.4 L20.5 17.6 L14.6 21.5"
        stroke={color}
        strokeWidth="3"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  );
}

/** Mark plus wordmark, set on the house face. */
export function BastionLockup({
  color = 'currentColor',
  markColor,
  size = 22,
}: {
  color?: string;
  markColor?: string;
  size?: number;
}) {
  return (
    <span style={{ display: 'inline-flex', alignItems: 'center', gap: '0.55rem' }}>
      <BastionMark size={size} color={markColor ?? color} />
      <span
        className="font-display"
        style={{ fontSize: `${size * 0.95}px`, fontWeight: 600, letterSpacing: '-0.03em', color }}
      >
        Bastion
      </span>
    </span>
  );
}
