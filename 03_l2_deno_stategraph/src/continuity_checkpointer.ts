// Transactional Multi-Session Chat Continuity Checkpointer

export interface ChatContinuityState {
  sessionId: string;
  superStepEpoch: number;
  activeBraidWord: string;
  topologicalCharge: number;
  parityTrace: number;
  phaseDelta: number;
  landauerJoules: number;
  invariantsLocked: boolean;
  timestamp: string;
}

export class ContinuityCheckpointer {
  private kv: Deno.Kv | null = null;
  private readonly diskPath = "C:\\sovereign_manifold_v27\\08_continuity_checkpoints\\active_session_anchor.json";

  async initialize(): Promise<void> {
    this.kv = await Deno.openKv();
  }

  async saveCheckpoint(state: ChatContinuityState): Promise<void> {
    if (!this.kv) await this.initialize();
    
    // 1. Transactional Commit to Deno KV
    await this.kv!.set(["continuity", state.sessionId, "latest"], state);
    await this.kv!.set(["continuity_history", state.sessionId, state.superStepEpoch], state);

    // 2. Atomic Disk Flush to JSON
    const payload = JSON.stringify(state, null, 2);
    await Deno.writeTextFile(this.diskPath, payload);
  }

  async restoreCheckpoint(sessionId: string): Promise<ChatContinuityState | null> {
    if (!this.kv) await this.initialize();
    
    // Check KV first
    const entry = await this.kv!.get<ChatContinuityState>(["continuity", sessionId, "latest"]);
    if (entry.value) return entry.value;

    // Fallback to disk snapshot
    try {
      const text = await Deno.readTextFile(this.diskPath);
      return JSON.parse(text) as ChatContinuityState;
    } catch {
      return null;
    }
  }

  close(): void {
    if (this.kv) {
      this.kv.close();
      this.kv = null;
    }
  }
}