declare type User = {
  id: string;
  avatar: string;
  username: string;
  discriminator: string;
};

declare type Activity = {
  state: string;
  details: string;
  timestamps: {
    start: number;
    end: number;
  };
  assets: {
    large_image: string;
    large_text: string;
    small_image: string;
    small_text: string;
  };
  buttons: { label: string; url: string }[];
};
