import { describe, expect, it } from 'vitest';
import {
  canRequestConnect,
  canRequestDisconnect,
  isTransitioning,
  requestedStatusAfterToggle,
} from './vpnStateMachine';

describe('vpnStateMachine', () => {
  it('allows connect only from disconnected or error', () => {
    expect(canRequestConnect('disconnected')).toBe(true);
    expect(canRequestConnect('error')).toBe(true);
    expect(canRequestConnect('connecting')).toBe(false);
    expect(canRequestConnect('connected')).toBe(false);
  });

  it('allows disconnect only from connected', () => {
    expect(canRequestDisconnect('connected')).toBe(true);
    expect(canRequestDisconnect('disconnected')).toBe(false);
  });

  it('maps toggle requests to transition statuses', () => {
    expect(requestedStatusAfterToggle('disconnected')).toBe('connecting');
    expect(requestedStatusAfterToggle('connected')).toBe('disconnecting');
    expect(requestedStatusAfterToggle('connecting')).toBe('connecting');
  });

  it('detects transition states', () => {
    expect(isTransitioning('connecting')).toBe(true);
    expect(isTransitioning('disconnecting')).toBe(true);
    expect(isTransitioning('connected')).toBe(false);
  });
});

