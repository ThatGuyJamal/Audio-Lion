import { writable, type Writable } from "svelte/store";

/** The user object for the current signed in app user */
interface UserSchema {
  id: string;
  name: string;
  username: string;
  email: string;
  hashedPassword: string;
  createdAt: string;
  updatedAt: string;
  deletedAt: string;
  tombStoned: boolean;
}

interface MusicSchema {}

/** The data for the player to handle. */
export interface PlayerStoreSchema {
  user: UserSchema;
  music: MusicSchema;
}

class PlayerStore {
  private static _store: Writable<PlayerStoreSchema>;

  public constructor() {
    PlayerStore._store = writable();
  }

  public setStore(store: PlayerStoreSchema) {
    PlayerStore._store.set(store);
  }
}

export const PlayerStoreInstance = new PlayerStore();
