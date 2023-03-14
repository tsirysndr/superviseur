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

export type Log = {
  __typename?: 'Log';
  lines: Array<Scalars['String']>;
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
  value: Scalars['String'];
};


export type MutationDeleteEnvVarArgs = {
  id: Scalars['ID'];
  name: Scalars['String'];
};


export type MutationRestartArgs = {
  id?: InputMaybe<Scalars['ID']>;
};


export type MutationStartArgs = {
  id?: InputMaybe<Scalars['ID']>;
};


export type MutationStopArgs = {
  id?: InputMaybe<Scalars['ID']>;
};


export type MutationUpdateEnvVarArgs = {
  id: Scalars['ID'];
  name: Scalars['String'];
  value: Scalars['String'];
};

export type Process = {
  __typename?: 'Process';
  autoRestart: Scalars['Boolean'];
  command: Scalars['String'];
  description: Scalars['String'];
  env: Array<Scalars['String']>;
  logFile: Scalars['String'];
  name: Scalars['String'];
  pid: Scalars['Int'];
  ppid: Scalars['Int'];
  project: Scalars['String'];
  stderrFile: Scalars['String'];
  type: Scalars['String'];
  workingDirectory: Scalars['String'];
};

export type Query = {
  __typename?: 'Query';
  logs: Log;
  processes: Array<Process>;
  service: Service;
  services: Array<Service>;
  status: Process;
  tail: Log;
};


export type QueryLogsArgs = {
  id: Scalars['ID'];
};


export type QueryServiceArgs = {
  id: Scalars['ID'];
};


export type QueryStatusArgs = {
  id: Scalars['ID'];
};


export type QueryTailArgs = {
  id: Scalars['ID'];
};

export type Service = {
  __typename?: 'Service';
  autoRestart: Scalars['Boolean'];
  command: Scalars['String'];
  dependsOn: Array<Scalars['String']>;
  description: Scalars['String'];
  env: Array<Scalars['String']>;
  id: Scalars['String'];
  logFile: Scalars['String'];
  name: Scalars['String'];
  namespace: Scalars['String'];
  port: Scalars['Int'];
  status: Scalars['String'];
  stderrFile: Scalars['String'];
  type: Scalars['String'];
  workingDirectory: Scalars['String'];
};

export type Subscription = {
  __typename?: 'Subscription';
  logs: Scalars['String'];
  tail: Scalars['String'];
};


export type SubscriptionLogsArgs = {
  id: Scalars['ID'];
};


export type SubscriptionTailArgs = {
  id: Scalars['ID'];
};

export type StartMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
}>;


export type StartMutation = { __typename?: 'Mutation', start: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type StopMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
}>;


export type StopMutation = { __typename?: 'Mutation', stop: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type RestartMutationVariables = Exact<{
  id?: InputMaybe<Scalars['ID']>;
}>;


export type RestartMutation = { __typename?: 'Mutation', restart: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type CreateEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
  value: Scalars['String'];
}>;


export type CreateEnvVarMutation = { __typename?: 'Mutation', createEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type DeleteEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
}>;


export type DeleteEnvVarMutation = { __typename?: 'Mutation', deleteEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type UpdateEnvVarMutationVariables = Exact<{
  id: Scalars['ID'];
  name: Scalars['String'];
  value: Scalars['String'];
}>;


export type UpdateEnvVarMutation = { __typename?: 'Mutation', updateEnvVar: { __typename?: 'Service', id: string, env: Array<string> } };

export type GetStatusQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type GetStatusQuery = { __typename?: 'Query', status: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type GetProcessesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetProcessesQuery = { __typename?: 'Query', processes: Array<{ __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> }> };

export type GetServicesQueryVariables = Exact<{ [key: string]: never; }>;


export type GetServicesQuery = { __typename?: 'Query', services: Array<{ __typename?: 'Service', id: string, name: string, command: string, description: string, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port: number }> };

export type GetServiceQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type GetServiceQuery = { __typename?: 'Query', service: { __typename?: 'Service', id: string, name: string, command: string, description: string, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port: number } };

export type ProcessFragmentFragment = { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> };

export type ServiceFragmentFragment = { __typename?: 'Service', id: string, name: string, command: string, description: string, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean, workingDirectory: string, logFile: string, stderrFile: string, port: number };

export type GetLogsQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type GetLogsQuery = { __typename?: 'Query', logs: { __typename?: 'Log', lines: Array<string> } };

export type TailLogsQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type TailLogsQuery = { __typename?: 'Query', tail: { __typename?: 'Log', lines: Array<string> } };

export type LogsSubscriptionVariables = Exact<{
  id: Scalars['ID'];
}>;


export type LogsSubscription = { __typename?: 'Subscription', logs: string };

export type TailSubscriptionVariables = Exact<{
  id: Scalars['ID'];
}>;


export type TailSubscription = { __typename?: 'Subscription', tail: string };

export const ProcessFragmentFragmentDoc = gql`
    fragment ProcessFragment on Process {
  name
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
    mutation Start($id: ID) {
  start(id: $id) {
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
    mutation Stop($id: ID) {
  stop(id: $id) {
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
    mutation Restart($id: ID) {
  restart(id: $id) {
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
    mutation CreateEnvVar($id: ID!, $name: String!, $value: String!) {
  createEnvVar(id: $id, name: $name, value: $value) {
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
    mutation DeleteEnvVar($id: ID!, $name: String!) {
  deleteEnvVar(id: $id, name: $name) {
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
    mutation UpdateEnvVar($id: ID!, $name: String!, $value: String!) {
  updateEnvVar(id: $id, name: $name, value: $value) {
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
    query GetServices {
  services {
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
 *   },
 * });
 */
export function useGetServicesQuery(baseOptions?: Apollo.QueryHookOptions<GetServicesQuery, GetServicesQueryVariables>) {
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
    query GetService($id: ID!) {
  service(id: $id) {
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
export const GetLogsDocument = gql`
    query GetLogs($id: ID!) {
  logs(id: $id) {
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
    query TailLogs($id: ID!) {
  tail(id: $id) {
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
    subscription Logs($id: ID!) {
  logs(id: $id)
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
    subscription Tail($id: ID!) {
  tail(id: $id)
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
 *   },
 * });
 */
export function useTailSubscription(baseOptions: Apollo.SubscriptionHookOptions<TailSubscription, TailSubscriptionVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useSubscription<TailSubscription, TailSubscriptionVariables>(TailDocument, options);
      }
export type TailSubscriptionHookResult = ReturnType<typeof useTailSubscription>;
export type TailSubscriptionResult = Apollo.SubscriptionResult<TailSubscription>;