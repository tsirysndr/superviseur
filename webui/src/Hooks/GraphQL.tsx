import { gql } from '@apollo/client';
import * as Apollo from '@apollo/client';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
const defaultOptions = {} as const;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: string;
  String: string;
  Boolean: boolean;
  Int: number;
  Float: number;
};

export type AllServicesRestarted = {
  __typename?: 'AllServicesRestarted';
  payload: Array<Service>;
};

export type AllServicesStarted = {
  __typename?: 'AllServicesStarted';
  payload: Array<Service>;
};

export type AllServicesStopped = {
  __typename?: 'AllServicesStopped';
  payload: Array<Service>;
};

export type Log = {
  __typename?: 'Log';
  lines: Array<Scalars['String']>;
};

export type LogStream = {
  __typename?: 'LogStream';
  line: Scalars['String'];
};

export type Mutation = {
  __typename?: 'Mutation';
  createEnvVar: Service;
  deleteEnvVar: Service;
  restart: Process;
  start: Process;
  stop: Process;
  updateEnvVar: Service;
};


export type MutationCreateEnvVarArgs = {
  id: Scalars['ID'];
  name: Scalars['String'];
  projectId: Scalars['ID'];
  value: Scalars['String'];
};


export type MutationDeleteEnvVarArgs = {
  id: Scalars['ID'];
  name: Scalars['String'];
  projectId: Scalars['ID'];
};


export type MutationRestartArgs = {
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
};


export type MutationStartArgs = {
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
};


export type MutationStopArgs = {
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
};


export type MutationUpdateEnvVarArgs = {
  id: Scalars['ID'];
  name: Scalars['String'];
  projectId: Scalars['ID'];
  value: Scalars['String'];
};

export type Process = {
  __typename?: 'Process';
  autoRestart: Scalars['Boolean'];
  command: Scalars['String'];
  description?: Maybe<Scalars['String']>;
  env: Array<Scalars['String']>;
  logFile: Scalars['String'];
  name: Scalars['String'];
  pid?: Maybe<Scalars['Int']>;
  ppid?: Maybe<Scalars['Int']>;
  project: Scalars['String'];
  serviceId: Scalars['ID'];
  state: Scalars['String'];
  stderrFile: Scalars['String'];
  type: Scalars['String'];
  upTime: Scalars['String'];
  workingDirectory: Scalars['String'];
};

export type Project = {
  __typename?: 'Project';
  configPath: Scalars['String'];
  id: Scalars['String'];
  name: Scalars['String'];
};

export type Query = {
  __typename?: 'Query';
  logs: Log;
  processes: Array<Process>;
  project: Project;
  projects: Array<Project>;
  service: Service;
  services: Array<Service>;
  status: Process;
  tail: Log;
};


export type QueryLogsArgs = {
  id: Scalars['ID'];
  projectId: Scalars['ID'];
};


export type QueryProjectArgs = {
  id: Scalars['ID'];
};


export type QueryServiceArgs = {
  id: Scalars['ID'];
  projectId: Scalars['ID'];
};


export type QueryServicesArgs = {
  projectId: Scalars['ID'];
};


export type QueryStatusArgs = {
  id: Scalars['ID'];
};


export type QueryTailArgs = {
  id: Scalars['ID'];
  numLines?: InputMaybe<Scalars['Int']>;
  projectId: Scalars['ID'];
};

export type Service = {
  __typename?: 'Service';
  autoRestart: Scalars['Boolean'];
  command: Scalars['String'];
  dependsOn: Array<Scalars['String']>;
  description?: Maybe<Scalars['String']>;
  env: Array<Scalars['String']>;
  id: Scalars['String'];
  logFile: Scalars['String'];
  name: Scalars['String'];
  namespace: Scalars['String'];
  port?: Maybe<Scalars['Int']>;
  status: Scalars['String'];
  stderrFile: Scalars['String'];
  type: Scalars['String'];
  workingDirectory: Scalars['String'];
};

export type ServiceRestarted = {
  __typename?: 'ServiceRestarted';
  payload: Service;
  process: Process;
};

export type ServiceStarted = {
  __typename?: 'ServiceStarted';
  payload: Service;
  process: Process;
};

export type ServiceStarting = {
  __typename?: 'ServiceStarting';
  payload: Service;
  process: Process;
};

export type ServiceStopped = {
  __typename?: 'ServiceStopped';
  payload: Service;
  process: Process;
};

export type ServiceStopping = {
  __typename?: 'ServiceStopping';
  payload: Service;
  process: Process;
};

export type Subscription = {
  __typename?: 'Subscription';
  logs: LogStream;
  onRestart: ServiceRestarted;
  onRestartAll: AllServicesRestarted;
  onStart: ServiceStarted;
  onStartAll: AllServicesStarted;
  onStarting: ServiceStarting;
  onStop: ServiceStopped;
  onStopAll: AllServicesStopped;
  onStopping: ServiceStopping;
  tail: TailLogStream;
};


export type SubscriptionLogsArgs = {
  id: Scalars['ID'];
  projectId: Scalars['ID'];
};


export type SubscriptionTailArgs = {
  id: Scalars['ID'];
  projectId: Scalars['ID'];
};

export type TailLogStream = {
  __typename?: 'TailLogStream';
  line: Scalars['String'];
};

export type StartMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
}>;


export type StartMutation = { __typename?: 'Mutation', start: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } };

export type StopMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
}>;


export type StopMutation = { __typename?: 'Mutation', stop: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } };

export type RestartMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
  projectId: Scalars['ID'];
}>;


export type RestartMutation = { __typename?: 'Mutation', restart: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } };

export type CreateEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
  value: Scalars['String'];
  projectId: Scalars['ID'];
}>;


export type CreateEnvVarMutation = { __typename?: 'Mutation', createEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type DeleteEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
  projectId: Scalars['ID'];
}>;


export type DeleteEnvVarMutation = { __typename?: 'Mutation', deleteEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type UpdateEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
  value: Scalars['String'];
  projectId: Scalars['ID'];
}>;


export type UpdateEnvVarMutation = { __typename?: 'Mutation', updateEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type GetStatusQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type GetStatusQuery = { __typename?: 'Query', status: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } };

export type GetProcessesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetProcessesQuery = { __typename?: 'Query', processes: Array<{ __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string }> };

export type GetServicesQueryVariables = Exact<{
  projectId: Scalars['ID'];
}>;


export type GetServicesQuery = { __typename?: 'Query', services: Array<{ __typename?: 'Service', id: string, name: string, command: string, description?: string | null, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port?: number | null }> };

export type GetServiceQueryVariables = Exact<{
  id: Scalars['ID'];
  projectId: Scalars['ID'];
}>;


export type GetServiceQuery = { __typename?: 'Query', service: { __typename?: 'Service', id: string, name: string, command: string, description?: string | null, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port?: number | null } };

export type GetEnvVarsQueryVariables = Exact<{
  id: Scalars['ID'];
  projectId: Scalars['ID'];
}>;


export type GetEnvVarsQuery = { __typename?: 'Query', service: { __typename?: 'Service', id: string, env: Array<string> } };

export type GetProjectsQueryVariables = Exact<{ [key: string]: never; }>;


export type GetProjectsQuery = { __typename?: 'Query', projects: Array<{ __typename?: 'Project', id: string, name: string, configPath: string }> };

export type GetProjectQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type GetProjectQuery = { __typename?: 'Query', project: { __typename?: 'Project', id: string, name: string, configPath: string } };

export type OnStartSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStartSubscription = { __typename?: 'Subscription', onStart: { __typename?: 'ServiceStarted', payload: { __typename?: 'Service', id: string, name: string, status: string }, process: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } } };

export type OnStopSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStopSubscription = { __typename?: 'Subscription', onStop: { __typename?: 'ServiceStopped', payload: { __typename?: 'Service', id: string, name: string, status: string }, process: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } } };

export type OnStartingSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStartingSubscription = { __typename?: 'Subscription', onStarting: { __typename?: 'ServiceStarting', payload: { __typename?: 'Service', id: string, name: string, status: string }, process: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } } };

export type OnStoppingSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStoppingSubscription = { __typename?: 'Subscription', onStopping: { __typename?: 'ServiceStopping', payload: { __typename?: 'Service', id: string, name: string, status: string }, process: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } } };

export type OnRestartSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnRestartSubscription = { __typename?: 'Subscription', onRestart: { __typename?: 'ServiceRestarted', payload: { __typename?: 'Service', id: string, name: string, status: string }, process: { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string } } };

export type OnStartAllSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStartAllSubscription = { __typename?: 'Subscription', onStartAll: { __typename?: 'AllServicesStarted', payload: Array<{ __typename?: 'Service', id: string, name: string, status: string }> } };

export type OnStopAllSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnStopAllSubscription = { __typename?: 'Subscription', onStopAll: { __typename?: 'AllServicesStopped', payload: Array<{ __typename?: 'Service', id: string, name: string, status: string }> } };

export type OnRestartAllSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type OnRestartAllSubscription = { __typename?: 'Subscription', onRestartAll: { __typename?: 'AllServicesRestarted', payload: Array<{ __typename?: 'Service', id: string, name: string, status: string }> } };

export type ProcessFragmentFragment = { __typename?: 'Process', name: string, serviceId: string, description?: string | null, pid?: number | null, ppid?: number | null, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string>, state: string, upTime: string };

export type ServiceFragmentFragment = { __typename?: 'Service', id: string, name: string, command: string, description?: string | null, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port?: number | null };

export type GetLogsQueryVariables = Exact<{
  id: Scalars['ID'];
  projectId: Scalars['ID'];
}>;


export type GetLogsQuery = { __typename?: 'Query', logs: { __typename?: 'Log', lines: Array<string> } };

export type TailLogsQueryVariables = Exact<{
  id: Scalars['ID'];
  numLines?: InputMaybe<Scalars['Int']>;
  projectId: Scalars['ID'];
}>;


export type TailLogsQuery = { __typename?: 'Query', tail: { __typename?: 'Log', lines: Array<string> } };

export type LogsSubscriptionVariables = Exact<{
  id: Scalars['ID'];
  projectId: Scalars['ID'];
}>;


export type LogsSubscription = { __typename?: 'Subscription', logs: { __typename?: 'LogStream', line: string } };

export type TailSubscriptionVariables = Exact<{
  id: Scalars['ID'];
  projectId: Scalars['ID'];
}>;


export type TailSubscription = { __typename?: 'Subscription', tail: { __typename?: 'TailLogStream', line: string } };

export const ProcessFragmentFragmentDoc = gql`
    fragment ProcessFragment on Process {
  name
  serviceId
  description
  pid
  ppid
  command
  workingDirectory
  project
  type
  logFile
  stderrFile
  autoRestart
  env
  state
  upTime
}
    `;
export const ServiceFragmentFragmentDoc = gql`
    fragment ServiceFragment on Service {
  id
  name
  command
  description
  namespace
  type
  status
  dependsOn
  env
  autoRestart
  workingDirectory
  logFile
  stderrFile
  port
}
    `;
export const StartDocument = gql`
    mutation Start($id: ID, $projectId: ID!) {
  start(id: $id, projectId: $projectId) {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;
export type StartMutationFn = Apollo.MutationFunction<StartMutation, StartMutationVariables>;

/**
 * __useStartMutation__
 *
 * To run a mutation, you first call `useStartMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useStartMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [startMutation, { data, loading, error }] = useStartMutation({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useStartMutation(baseOptions?: Apollo.MutationHookOptions<StartMutation, StartMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<StartMutation, StartMutationVariables>(StartDocument, options);
      }
export type StartMutationHookResult = ReturnType<typeof useStartMutation>;
export type StartMutationResult = Apollo.MutationResult<StartMutation>;
export type StartMutationOptions = Apollo.BaseMutationOptions<StartMutation, StartMutationVariables>;
export const StopDocument = gql`
    mutation Stop($id: ID, $projectId: ID!) {
  stop(id: $id, projectId: $projectId) {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;
export type StopMutationFn = Apollo.MutationFunction<StopMutation, StopMutationVariables>;

/**
 * __useStopMutation__
 *
 * To run a mutation, you first call `useStopMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useStopMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [stopMutation, { data, loading, error }] = useStopMutation({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useStopMutation(baseOptions?: Apollo.MutationHookOptions<StopMutation, StopMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<StopMutation, StopMutationVariables>(StopDocument, options);
      }
export type StopMutationHookResult = ReturnType<typeof useStopMutation>;
export type StopMutationResult = Apollo.MutationResult<StopMutation>;
export type StopMutationOptions = Apollo.BaseMutationOptions<StopMutation, StopMutationVariables>;
export const RestartDocument = gql`
    mutation Restart($id: ID, $projectId: ID!) {
  restart(id: $id, projectId: $projectId) {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;
export type RestartMutationFn = Apollo.MutationFunction<RestartMutation, RestartMutationVariables>;

/**
 * __useRestartMutation__
 *
 * To run a mutation, you first call `useRestartMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useRestartMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [restartMutation, { data, loading, error }] = useRestartMutation({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useRestartMutation(baseOptions?: Apollo.MutationHookOptions<RestartMutation, RestartMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<RestartMutation, RestartMutationVariables>(RestartDocument, options);
      }
export type RestartMutationHookResult = ReturnType<typeof useRestartMutation>;
export type RestartMutationResult = Apollo.MutationResult<RestartMutation>;
export type RestartMutationOptions = Apollo.BaseMutationOptions<RestartMutation, RestartMutationVariables>;
export const CreateEnvVarDocument = gql`
    mutation CreateEnvVar($id: ID!, $name: String!, $value: String!, $projectId: ID!) {
  createEnvVar(id: $id, name: $name, value: $value, projectId: $projectId) {
    id
    env
  }
}
    `;
export type CreateEnvVarMutationFn = Apollo.MutationFunction<CreateEnvVarMutation, CreateEnvVarMutationVariables>;

/**
 * __useCreateEnvVarMutation__
 *
 * To run a mutation, you first call `useCreateEnvVarMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useCreateEnvVarMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [createEnvVarMutation, { data, loading, error }] = useCreateEnvVarMutation({
 *   variables: {
 *      id: // value for 'id'
 *      name: // value for 'name'
 *      value: // value for 'value'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useCreateEnvVarMutation(baseOptions?: Apollo.MutationHookOptions<CreateEnvVarMutation, CreateEnvVarMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<CreateEnvVarMutation, CreateEnvVarMutationVariables>(CreateEnvVarDocument, options);
      }
export type CreateEnvVarMutationHookResult = ReturnType<typeof useCreateEnvVarMutation>;
export type CreateEnvVarMutationResult = Apollo.MutationResult<CreateEnvVarMutation>;
export type CreateEnvVarMutationOptions = Apollo.BaseMutationOptions<CreateEnvVarMutation, CreateEnvVarMutationVariables>;
export const DeleteEnvVarDocument = gql`
    mutation DeleteEnvVar($id: ID!, $name: String!, $projectId: ID!) {
  deleteEnvVar(id: $id, name: $name, projectId: $projectId) {
    id
    env
  }
}
    `;
export type DeleteEnvVarMutationFn = Apollo.MutationFunction<DeleteEnvVarMutation, DeleteEnvVarMutationVariables>;

/**
 * __useDeleteEnvVarMutation__
 *
 * To run a mutation, you first call `useDeleteEnvVarMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useDeleteEnvVarMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [deleteEnvVarMutation, { data, loading, error }] = useDeleteEnvVarMutation({
 *   variables: {
 *      id: // value for 'id'
 *      name: // value for 'name'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useDeleteEnvVarMutation(baseOptions?: Apollo.MutationHookOptions<DeleteEnvVarMutation, DeleteEnvVarMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<DeleteEnvVarMutation, DeleteEnvVarMutationVariables>(DeleteEnvVarDocument, options);
      }
export type DeleteEnvVarMutationHookResult = ReturnType<typeof useDeleteEnvVarMutation>;
export type DeleteEnvVarMutationResult = Apollo.MutationResult<DeleteEnvVarMutation>;
export type DeleteEnvVarMutationOptions = Apollo.BaseMutationOptions<DeleteEnvVarMutation, DeleteEnvVarMutationVariables>;
export const UpdateEnvVarDocument = gql`
    mutation UpdateEnvVar($id: ID!, $name: String!, $value: String!, $projectId: ID!) {
  updateEnvVar(id: $id, name: $name, value: $value, projectId: $projectId) {
    id
    env
  }
}
    `;
export type UpdateEnvVarMutationFn = Apollo.MutationFunction<UpdateEnvVarMutation, UpdateEnvVarMutationVariables>;

/**
 * __useUpdateEnvVarMutation__
 *
 * To run a mutation, you first call `useUpdateEnvVarMutation` within a React component and pass it any options that fit your needs.
 * When your component renders, `useUpdateEnvVarMutation` returns a tuple that includes:
 * - A mutate function that you can call at any time to execute the mutation
 * - An object with fields that represent the current status of the mutation's execution
 *
 * @param baseOptions options that will be passed into the mutation, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options-2;
 *
 * @example
 * const [updateEnvVarMutation, { data, loading, error }] = useUpdateEnvVarMutation({
 *   variables: {
 *      id: // value for 'id'
 *      name: // value for 'name'
 *      value: // value for 'value'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useUpdateEnvVarMutation(baseOptions?: Apollo.MutationHookOptions<UpdateEnvVarMutation, UpdateEnvVarMutationVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useMutation<UpdateEnvVarMutation, UpdateEnvVarMutationVariables>(UpdateEnvVarDocument, options);
      }
export type UpdateEnvVarMutationHookResult = ReturnType<typeof useUpdateEnvVarMutation>;
export type UpdateEnvVarMutationResult = Apollo.MutationResult<UpdateEnvVarMutation>;
export type UpdateEnvVarMutationOptions = Apollo.BaseMutationOptions<UpdateEnvVarMutation, UpdateEnvVarMutationVariables>;
export const GetStatusDocument = gql`
    query GetStatus($id: ID!) {
  status(id: $id) {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useGetStatusQuery__
 *
 * To run a query within a React component, call `useGetStatusQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetStatusQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetStatusQuery({
 *   variables: {
 *      id: // value for 'id'
 *   },
 * });
 */
export function useGetStatusQuery(baseOptions: Apollo.QueryHookOptions<GetStatusQuery, GetStatusQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetStatusQuery, GetStatusQueryVariables>(GetStatusDocument, options);
      }
export function useGetStatusLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetStatusQuery, GetStatusQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetStatusQuery, GetStatusQueryVariables>(GetStatusDocument, options);
        }
export type GetStatusQueryHookResult = ReturnType<typeof useGetStatusQuery>;
export type GetStatusLazyQueryHookResult = ReturnType<typeof useGetStatusLazyQuery>;
export type GetStatusQueryResult = Apollo.QueryResult<GetStatusQuery, GetStatusQueryVariables>;
export const GetProcessesDocument = gql`
    query GetProcesses {
  processes {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useGetProcessesQuery__
 *
 * To run a query within a React component, call `useGetProcessesQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetProcessesQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetProcessesQuery({
 *   variables: {
 *   },
 * });
 */
export function useGetProcessesQuery(baseOptions?: Apollo.QueryHookOptions<GetProcessesQuery, GetProcessesQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetProcessesQuery, GetProcessesQueryVariables>(GetProcessesDocument, options);
      }
export function useGetProcessesLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetProcessesQuery, GetProcessesQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetProcessesQuery, GetProcessesQueryVariables>(GetProcessesDocument, options);
        }
export type GetProcessesQueryHookResult = ReturnType<typeof useGetProcessesQuery>;
export type GetProcessesLazyQueryHookResult = ReturnType<typeof useGetProcessesLazyQuery>;
export type GetProcessesQueryResult = Apollo.QueryResult<GetProcessesQuery, GetProcessesQueryVariables>;
export const GetServicesDocument = gql`
    query GetServices($projectId: ID!) {
  services(projectId: $projectId) {
    ...ServiceFragment
  }
}
    ${ServiceFragmentFragmentDoc}`;

/**
 * __useGetServicesQuery__
 *
 * To run a query within a React component, call `useGetServicesQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetServicesQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetServicesQuery({
 *   variables: {
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useGetServicesQuery(baseOptions: Apollo.QueryHookOptions<GetServicesQuery, GetServicesQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetServicesQuery, GetServicesQueryVariables>(GetServicesDocument, options);
      }
export function useGetServicesLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetServicesQuery, GetServicesQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetServicesQuery, GetServicesQueryVariables>(GetServicesDocument, options);
        }
export type GetServicesQueryHookResult = ReturnType<typeof useGetServicesQuery>;
export type GetServicesLazyQueryHookResult = ReturnType<typeof useGetServicesLazyQuery>;
export type GetServicesQueryResult = Apollo.QueryResult<GetServicesQuery, GetServicesQueryVariables>;
export const GetServiceDocument = gql`
    query GetService($id: ID!, $projectId: ID!) {
  service(id: $id, projectId: $projectId) {
    ...ServiceFragment
  }
}
    ${ServiceFragmentFragmentDoc}`;

/**
 * __useGetServiceQuery__
 *
 * To run a query within a React component, call `useGetServiceQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetServiceQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetServiceQuery({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useGetServiceQuery(baseOptions: Apollo.QueryHookOptions<GetServiceQuery, GetServiceQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetServiceQuery, GetServiceQueryVariables>(GetServiceDocument, options);
      }
export function useGetServiceLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetServiceQuery, GetServiceQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetServiceQuery, GetServiceQueryVariables>(GetServiceDocument, options);
        }
export type GetServiceQueryHookResult = ReturnType<typeof useGetServiceQuery>;
export type GetServiceLazyQueryHookResult = ReturnType<typeof useGetServiceLazyQuery>;
export type GetServiceQueryResult = Apollo.QueryResult<GetServiceQuery, GetServiceQueryVariables>;
export const GetEnvVarsDocument = gql`
    query GetEnvVars($id: ID!, $projectId: ID!) {
  service(id: $id, projectId: $projectId) {
    id
    env
  }
}
    `;

/**
 * __useGetEnvVarsQuery__
 *
 * To run a query within a React component, call `useGetEnvVarsQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetEnvVarsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetEnvVarsQuery({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useGetEnvVarsQuery(baseOptions: Apollo.QueryHookOptions<GetEnvVarsQuery, GetEnvVarsQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetEnvVarsQuery, GetEnvVarsQueryVariables>(GetEnvVarsDocument, options);
      }
export function useGetEnvVarsLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetEnvVarsQuery, GetEnvVarsQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetEnvVarsQuery, GetEnvVarsQueryVariables>(GetEnvVarsDocument, options);
        }
export type GetEnvVarsQueryHookResult = ReturnType<typeof useGetEnvVarsQuery>;
export type GetEnvVarsLazyQueryHookResult = ReturnType<typeof useGetEnvVarsLazyQuery>;
export type GetEnvVarsQueryResult = Apollo.QueryResult<GetEnvVarsQuery, GetEnvVarsQueryVariables>;
export const GetProjectsDocument = gql`
    query GetProjects {
  projects {
    id
    name
    configPath
  }
}
    `;

/**
 * __useGetProjectsQuery__
 *
 * To run a query within a React component, call `useGetProjectsQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetProjectsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetProjectsQuery({
 *   variables: {
 *   },
 * });
 */
export function useGetProjectsQuery(baseOptions?: Apollo.QueryHookOptions<GetProjectsQuery, GetProjectsQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetProjectsQuery, GetProjectsQueryVariables>(GetProjectsDocument, options);
      }
export function useGetProjectsLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetProjectsQuery, GetProjectsQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetProjectsQuery, GetProjectsQueryVariables>(GetProjectsDocument, options);
        }
export type GetProjectsQueryHookResult = ReturnType<typeof useGetProjectsQuery>;
export type GetProjectsLazyQueryHookResult = ReturnType<typeof useGetProjectsLazyQuery>;
export type GetProjectsQueryResult = Apollo.QueryResult<GetProjectsQuery, GetProjectsQueryVariables>;
export const GetProjectDocument = gql`
    query GetProject($id: ID!) {
  project(id: $id) {
    id
    name
    configPath
  }
}
    `;

/**
 * __useGetProjectQuery__
 *
 * To run a query within a React component, call `useGetProjectQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetProjectQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetProjectQuery({
 *   variables: {
 *      id: // value for 'id'
 *   },
 * });
 */
export function useGetProjectQuery(baseOptions: Apollo.QueryHookOptions<GetProjectQuery, GetProjectQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetProjectQuery, GetProjectQueryVariables>(GetProjectDocument, options);
      }
export function useGetProjectLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetProjectQuery, GetProjectQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetProjectQuery, GetProjectQueryVariables>(GetProjectDocument, options);
        }
export type GetProjectQueryHookResult = ReturnType<typeof useGetProjectQuery>;
export type GetProjectLazyQueryHookResult = ReturnType<typeof useGetProjectLazyQuery>;
export type GetProjectQueryResult = Apollo.QueryResult<GetProjectQuery, GetProjectQueryVariables>;
export const OnStartDocument = gql`
    subscription OnStart {
  onStart {
    payload {
      id
      name
      status
    }
    process {
      ...ProcessFragment
    }
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useOnStartSubscription__
 *
 * To run a query within a React component, call `useOnStartSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStartSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStartSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStartSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStartSubscription, OnStartSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStartSubscription, OnStartSubscriptionVariables>(OnStartDocument, options);
      }
export type OnStartSubscriptionHookResult = ReturnType<typeof useOnStartSubscription>;
export type OnStartSubscriptionResult = Apollo.SubscriptionResult<OnStartSubscription>;
export const OnStopDocument = gql`
    subscription OnStop {
  onStop {
    payload {
      id
      name
      status
    }
    process {
      ...ProcessFragment
    }
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useOnStopSubscription__
 *
 * To run a query within a React component, call `useOnStopSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStopSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStopSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStopSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStopSubscription, OnStopSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStopSubscription, OnStopSubscriptionVariables>(OnStopDocument, options);
      }
export type OnStopSubscriptionHookResult = ReturnType<typeof useOnStopSubscription>;
export type OnStopSubscriptionResult = Apollo.SubscriptionResult<OnStopSubscription>;
export const OnStartingDocument = gql`
    subscription OnStarting {
  onStarting {
    payload {
      id
      name
      status
    }
    process {
      ...ProcessFragment
    }
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useOnStartingSubscription__
 *
 * To run a query within a React component, call `useOnStartingSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStartingSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStartingSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStartingSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStartingSubscription, OnStartingSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStartingSubscription, OnStartingSubscriptionVariables>(OnStartingDocument, options);
      }
export type OnStartingSubscriptionHookResult = ReturnType<typeof useOnStartingSubscription>;
export type OnStartingSubscriptionResult = Apollo.SubscriptionResult<OnStartingSubscription>;
export const OnStoppingDocument = gql`
    subscription OnStopping {
  onStopping {
    payload {
      id
      name
      status
    }
    process {
      ...ProcessFragment
    }
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useOnStoppingSubscription__
 *
 * To run a query within a React component, call `useOnStoppingSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStoppingSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStoppingSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStoppingSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStoppingSubscription, OnStoppingSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStoppingSubscription, OnStoppingSubscriptionVariables>(OnStoppingDocument, options);
      }
export type OnStoppingSubscriptionHookResult = ReturnType<typeof useOnStoppingSubscription>;
export type OnStoppingSubscriptionResult = Apollo.SubscriptionResult<OnStoppingSubscription>;
export const OnRestartDocument = gql`
    subscription OnRestart {
  onRestart {
    payload {
      id
      name
      status
    }
    process {
      ...ProcessFragment
    }
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useOnRestartSubscription__
 *
 * To run a query within a React component, call `useOnRestartSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnRestartSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnRestartSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnRestartSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnRestartSubscription, OnRestartSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnRestartSubscription, OnRestartSubscriptionVariables>(OnRestartDocument, options);
      }
export type OnRestartSubscriptionHookResult = ReturnType<typeof useOnRestartSubscription>;
export type OnRestartSubscriptionResult = Apollo.SubscriptionResult<OnRestartSubscription>;
export const OnStartAllDocument = gql`
    subscription OnStartAll {
  onStartAll {
    payload {
      id
      name
      status
    }
  }
}
    `;

/**
 * __useOnStartAllSubscription__
 *
 * To run a query within a React component, call `useOnStartAllSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStartAllSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStartAllSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStartAllSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStartAllSubscription, OnStartAllSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStartAllSubscription, OnStartAllSubscriptionVariables>(OnStartAllDocument, options);
      }
export type OnStartAllSubscriptionHookResult = ReturnType<typeof useOnStartAllSubscription>;
export type OnStartAllSubscriptionResult = Apollo.SubscriptionResult<OnStartAllSubscription>;
export const OnStopAllDocument = gql`
    subscription OnStopAll {
  onStopAll {
    payload {
      id
      name
      status
    }
  }
}
    `;

/**
 * __useOnStopAllSubscription__
 *
 * To run a query within a React component, call `useOnStopAllSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnStopAllSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnStopAllSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnStopAllSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnStopAllSubscription, OnStopAllSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnStopAllSubscription, OnStopAllSubscriptionVariables>(OnStopAllDocument, options);
      }
export type OnStopAllSubscriptionHookResult = ReturnType<typeof useOnStopAllSubscription>;
export type OnStopAllSubscriptionResult = Apollo.SubscriptionResult<OnStopAllSubscription>;
export const OnRestartAllDocument = gql`
    subscription OnRestartAll {
  onRestartAll {
    payload {
      id
      name
      status
    }
  }
}
    `;

/**
 * __useOnRestartAllSubscription__
 *
 * To run a query within a React component, call `useOnRestartAllSubscription` and pass it any options that fit your needs.
 * When your component renders, `useOnRestartAllSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useOnRestartAllSubscription({
 *   variables: {
 *   },
 * });
 */
export function useOnRestartAllSubscription(baseOptions?: Apollo.SubscriptionHookOptions<OnRestartAllSubscription, OnRestartAllSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<OnRestartAllSubscription, OnRestartAllSubscriptionVariables>(OnRestartAllDocument, options);
      }
export type OnRestartAllSubscriptionHookResult = ReturnType<typeof useOnRestartAllSubscription>;
export type OnRestartAllSubscriptionResult = Apollo.SubscriptionResult<OnRestartAllSubscription>;
export const GetLogsDocument = gql`
    query GetLogs($id: ID!, $projectId: ID!) {
  logs(id: $id, projectId: $projectId) {
    lines
  }
}
    `;

/**
 * __useGetLogsQuery__
 *
 * To run a query within a React component, call `useGetLogsQuery` and pass it any options that fit your needs.
 * When your component renders, `useGetLogsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useGetLogsQuery({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useGetLogsQuery(baseOptions: Apollo.QueryHookOptions<GetLogsQuery, GetLogsQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<GetLogsQuery, GetLogsQueryVariables>(GetLogsDocument, options);
      }
export function useGetLogsLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<GetLogsQuery, GetLogsQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<GetLogsQuery, GetLogsQueryVariables>(GetLogsDocument, options);
        }
export type GetLogsQueryHookResult = ReturnType<typeof useGetLogsQuery>;
export type GetLogsLazyQueryHookResult = ReturnType<typeof useGetLogsLazyQuery>;
export type GetLogsQueryResult = Apollo.QueryResult<GetLogsQuery, GetLogsQueryVariables>;
export const TailLogsDocument = gql`
    query TailLogs($id: ID!, $numLines: Int, $projectId: ID!) {
  tail(id: $id, numLines: $numLines, projectId: $projectId) {
    lines
  }
}
    `;

/**
 * __useTailLogsQuery__
 *
 * To run a query within a React component, call `useTailLogsQuery` and pass it any options that fit your needs.
 * When your component renders, `useTailLogsQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useTailLogsQuery({
 *   variables: {
 *      id: // value for 'id'
 *      numLines: // value for 'numLines'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useTailLogsQuery(baseOptions: Apollo.QueryHookOptions<TailLogsQuery, TailLogsQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<TailLogsQuery, TailLogsQueryVariables>(TailLogsDocument, options);
      }
export function useTailLogsLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<TailLogsQuery, TailLogsQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<TailLogsQuery, TailLogsQueryVariables>(TailLogsDocument, options);
        }
export type TailLogsQueryHookResult = ReturnType<typeof useTailLogsQuery>;
export type TailLogsLazyQueryHookResult = ReturnType<typeof useTailLogsLazyQuery>;
export type TailLogsQueryResult = Apollo.QueryResult<TailLogsQuery, TailLogsQueryVariables>;
export const LogsDocument = gql`
    subscription Logs($id: ID!, $projectId: ID!) {
  logs(id: $id, projectId: $projectId) {
    line
  }
}
    `;

/**
 * __useLogsSubscription__
 *
 * To run a query within a React component, call `useLogsSubscription` and pass it any options that fit your needs.
 * When your component renders, `useLogsSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useLogsSubscription({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useLogsSubscription(baseOptions: Apollo.SubscriptionHookOptions<LogsSubscription, LogsSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<LogsSubscription, LogsSubscriptionVariables>(LogsDocument, options);
      }
export type LogsSubscriptionHookResult = ReturnType<typeof useLogsSubscription>;
export type LogsSubscriptionResult = Apollo.SubscriptionResult<LogsSubscription>;
export const TailDocument = gql`
    subscription Tail($id: ID!, $projectId: ID!) {
  tail(id: $id, projectId: $projectId) {
    line
  }
}
    `;

/**
 * __useTailSubscription__
 *
 * To run a query within a React component, call `useTailSubscription` and pass it any options that fit your needs.
 * When your component renders, `useTailSubscription` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the subscription, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useTailSubscription({
 *   variables: {
 *      id: // value for 'id'
 *      projectId: // value for 'projectId'
 *   },
 * });
 */
export function useTailSubscription(baseOptions: Apollo.SubscriptionHookOptions<TailSubscription, TailSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<TailSubscription, TailSubscriptionVariables>(TailDocument, options);
      }
export type TailSubscriptionHookResult = ReturnType<typeof useTailSubscription>;
export type TailSubscriptionResult = Apollo.SubscriptionResult<TailSubscription>;