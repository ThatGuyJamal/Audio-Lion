/**
 * This type represents the data that is returned from the a file loaded from the system.
 */
export type FetchSystemFile = {
  /**
   * The relative file name and type
   * @example "audio.mp3"
   */
  name: string;
  /** The last modified date from the file system */
  lastModified: number;

  lastModifiedDate: Date;
  webkitRelativePath: string;
  size: number;
  type: string;
};

export type FetchSystemFileResponse = FetchSystemFile[];
