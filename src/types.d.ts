declare type User = Partial<{
  id: string;
  avatar: string;
  username: string;
  discriminator: string;
}>;

declare type Activity = Partial<{
  state: string;
  details: string;
  timestamps: Partial<{
    start: number;
    end: number;
  }>;
  assets: Partial<{
    large_image: string;
    large_image_key: string;
    small_image: string;
    small_image_key: string;
  }>;
  party: {
    size?: [number, number];
  };
}>;
