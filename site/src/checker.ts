export type DemoFinding = {
  code: 'orphaned' | 'missing' | 'stale' | 'uncovered';
  path: string;
  message: string;
};

export type DemoResult = {
  valid: boolean;
  empty: boolean;
  decisions: number;
  findings: DemoFinding[];
};

type Decision = { path?: unknown; rationale?: unknown; valueHash?: unknown };
type Rule = { pattern?: unknown; minimumCoverage?: unknown };
type Sidecar = { decisions?: unknown; rules?: unknown };

function canonicalize(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (value !== null && typeof value === 'object') {
    return Object.fromEntries(
      Object.entries(value as Record<string, unknown>)
        .sort(([left], [right]) => left.localeCompare(right))
        .map(([key, child]) => [key, canonicalize(child)])
    );
  }
  return value;
}

export async function fingerprint(value: unknown): Promise<string> {
  const data = new TextEncoder().encode(JSON.stringify(canonicalize(value)));
  const digest = await crypto.subtle.digest('SHA-256', data);
  const hex = Array.from(new Uint8Array(digest), byte => byte.toString(16).padStart(2, '0')).join('');
  return `sha256:${hex}`;
}

function escapePointer(value: string): string {
  return value.replaceAll('~', '~0').replaceAll('/', '~1');
}

export function leafPaths(value: unknown, path = ''): string[] {
  if (Array.isArray(value) && value.length > 0) {
    return value.flatMap((child, index) => leafPaths(child, `${path}/${index}`));
  }
  if (value !== null && typeof value === 'object' && Object.keys(value).length > 0) {
    return Object.entries(value as Record<string, unknown>).flatMap(([key, child]) =>
      leafPaths(child, `${path}/${escapePointer(key)}`)
    );
  }
  return [path];
}

export function pointer(value: unknown, path: string): { found: boolean; value?: unknown } {
  if (path === '') return { found: true, value };
  if (!path.startsWith('/')) return { found: false };
  let current = value;
  for (const encoded of path.slice(1).split('/')) {
    const segment = encoded.replaceAll('~1', '/').replaceAll('~0', '~');
    if (Array.isArray(current)) {
      if (!/^\d+$/.test(segment) || Number(segment) >= current.length) return { found: false };
      current = current[Number(segment)];
    } else if (current !== null && typeof current === 'object' && Object.hasOwn(current, segment)) {
      current = (current as Record<string, unknown>)[segment];
    } else {
      return { found: false };
    }
  }
  return { found: true, value: current };
}

export function patternMatches(pattern: string, path: string): boolean {
  const patterns = pattern.replace(/^\//, '').split('/');
  const paths = path.replace(/^\//, '').split('/');
  const match = (patternIndex: number, pathIndex: number): boolean => {
    if (patternIndex === patterns.length) return pathIndex === paths.length;
    if (patterns[patternIndex] === '**') {
      return match(patternIndex + 1, pathIndex) || (pathIndex < paths.length && match(patternIndex, pathIndex + 1));
    }
    if (pathIndex === paths.length) return false;
    if (patterns[patternIndex] === '*' || patterns[patternIndex] === paths[pathIndex]) {
      return match(patternIndex + 1, pathIndex + 1);
    }
    return false;
  };
  return match(0, 0);
}

function usefulRationale(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length >= 12 && value.trim().toLowerCase() !== 'todo';
}

function decisionsOf(sidecar: Sidecar): Decision[] {
  return Array.isArray(sidecar.decisions) ? sidecar.decisions : [];
}

function rulesOf(sidecar: Sidecar): Rule[] {
  return Array.isArray(sidecar.rules) ? sidecar.rules : [];
}

export async function check(config: unknown, sidecar: Sidecar): Promise<DemoResult> {
  const leaves = leafPaths(config);
  const trulyEmpty = config !== null && typeof config === 'object' && Object.keys(config).length === 0;
  const findings: DemoFinding[] = [];
  const decisions = decisionsOf(sidecar);
  const decisionsByPath = new Map<string, Decision>();

  for (const decision of decisions) {
    if (typeof decision.path !== 'string') continue;
    decisionsByPath.set(decision.path, decision);
    const target = pointer(config, decision.path);
    if (!target.found) {
      findings.push({ code: 'orphaned', path: decision.path, message: 'Target no longer exists.' });
      continue;
    }
    if (!usefulRationale(decision.rationale)) {
      findings.push({ code: 'missing', path: decision.path, message: 'Add a concrete rationale of at least 12 characters.' });
    }
    if (decision.valueHash !== await fingerprint(target.value)) {
      findings.push({ code: 'stale', path: decision.path, message: 'Value changed since this decision was reviewed.' });
    }
  }

  for (const rule of rulesOf(sidecar)) {
    if (typeof rule.pattern !== 'string') continue;
    const targets = leaves.filter(path => patternMatches(rule.pattern as string, path));
    const covered = targets.filter(path => usefulRationale(decisionsByPath.get(path)?.rationale)).length;
    const minimum = typeof rule.minimumCoverage === 'number' ? rule.minimumCoverage : 1;
    if (targets.length > 0 && covered / targets.length < minimum) {
      findings.push({ code: 'uncovered', path: rule.pattern, message: `${covered}/${targets.length} policy-sensitive paths have rationale.` });
    }
  }

  return { valid: findings.length === 0, empty: trulyEmpty, decisions: decisions.length, findings };
}

export async function stamp(config: unknown, sidecar: Sidecar): Promise<Sidecar> {
  const decisions = decisionsOf(sidecar);
  for (const decision of decisions) {
    if (typeof decision.path !== 'string' || !usefulRationale(decision.rationale)) continue;
    const target = pointer(config, decision.path);
    if (target.found) decision.valueHash = await fingerprint(target.value);
  }
  sidecar.decisions = decisions;
  return sidecar;
}
