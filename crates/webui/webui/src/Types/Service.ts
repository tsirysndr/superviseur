export type Dependency = {
  source: string;
  target: string;
};

export type Service = {
  id: string;
  name: string;
  description: string;
  status: string;
  workingDirectory: string;
  command: string;
  dependencies: Dependency[];
};
