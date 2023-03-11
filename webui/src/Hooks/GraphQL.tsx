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
  restart: Process;
  start: Process;
  stop: Process;
};


export type MutationRestartArgs = {
  id: Scalars['ID'];
};


export type MutationStartArgs = {
  id: Scalars['ID'];
};


export type MutationStopArgs = {
  id: Scalars['ID'];
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
  services: Array<Service>;
  status: Process;
  tail: Log;
};


export type QueryLogsArgs = {
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
  name: Scalars['String'];
  namespace: Scalars['String'];
  status: Scalars['String'];
  type: Scalars['String'];
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
  id: Scalars['ID'];
}>;


export type StartMutation = { __typename?: 'Mutation', start: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type StopMutationVariables = Exact<{
  id: Scalars['ID'];
}>;


export type StopMutation = { __typename?: 'Mutation', stop: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type RestartMutationVariables = Exact<{
  id: Scalars['ID'];
}>;


export type RestartMutation = { __typename?: 'Mutation', restart: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type StatusQueryVariables = Exact<{
  id: Scalars['ID'];
}>;


export type StatusQuery = { __typename?: 'Query', status: { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> } };

export type ProcessesQueryVariables = Exact<{ [key: string]: never; }>;


export type ProcessesQuery = { __typename?: 'Query', processes: Array<{ __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> }> };

export type ServicesQueryVariables = Exact<{ [key: string]: never; }>;


export type ServicesQuery = { __typename?: 'Query', services: Array<{ __typename?: 'Service', name: string, command: string, description: string, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean }> };

export type ProcessFragmentFragment = { __typename?: 'Process', name: string, description: string, pid: number, ppid: number, command: string, workingDirectory: string, project: string, type: string, logFile: string, stderrFile: string, autoRestart: boolean, env: Array<string> };

export type ServiceFragmentFragment = { __typename?: 'Service', name: string, command: string, description: string, namespace: string, type: string, status: string, dependsOn: Array<string>, env: Array<string>, autoRestart: boolean };

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
  name
  command
  description
  namespace
  type
  status
  dependsOn
  env
  autoRestart
}
    `;
export const StartDocument = gql`
    mutation Start($id: ID!) {
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
    mutation Stop($id: ID!) {
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
    mutation Restart($id: ID!) {
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
export const StatusDocument = gql`
    query Status($id: ID!) {
  status(id: $id) {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useStatusQuery__
 *
 * To run a query within a React component, call `useStatusQuery` and pass it any options that fit your needs.
 * When your component renders, `useStatusQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useStatusQuery({
 *   variables: {
 *      id: // value for 'id'
 *   },
 * });
 */
export function useStatusQuery(baseOptions: Apollo.QueryHookOptions<StatusQuery, StatusQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<StatusQuery, StatusQueryVariables>(StatusDocument, options);
      }
export function useStatusLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<StatusQuery, StatusQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<StatusQuery, StatusQueryVariables>(StatusDocument, options);
        }
export type StatusQueryHookResult = ReturnType<typeof useStatusQuery>;
export type StatusLazyQueryHookResult = ReturnType<typeof useStatusLazyQuery>;
export type StatusQueryResult = Apollo.QueryResult<StatusQuery, StatusQueryVariables>;
export const ProcessesDocument = gql`
    query Processes {
  processes {
    ...ProcessFragment
  }
}
    ${ProcessFragmentFragmentDoc}`;

/**
 * __useProcessesQuery__
 *
 * To run a query within a React component, call `useProcessesQuery` and pass it any options that fit your needs.
 * When your component renders, `useProcessesQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useProcessesQuery({
 *   variables: {
 *   },
 * });
 */
export function useProcessesQuery(baseOptions?: Apollo.QueryHookOptions<ProcessesQuery, ProcessesQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<ProcessesQuery, ProcessesQueryVariables>(ProcessesDocument, options);
      }
export function useProcessesLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<ProcessesQuery, ProcessesQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<ProcessesQuery, ProcessesQueryVariables>(ProcessesDocument, options);
        }
export type ProcessesQueryHookResult = ReturnType<typeof useProcessesQuery>;
export type ProcessesLazyQueryHookResult = ReturnType<typeof useProcessesLazyQuery>;
export type ProcessesQueryResult = Apollo.QueryResult<ProcessesQuery, ProcessesQueryVariables>;
export const ServicesDocument = gql`
    query Services {
  services {
    ...ServiceFragment
  }
}
    ${ServiceFragmentFragmentDoc}`;

/**
 * __useServicesQuery__
 *
 * To run a query within a React component, call `useServicesQuery` and pass it any options that fit your needs.
 * When your component renders, `useServicesQuery` returns an object from Apollo Client that contains loading, error, and data properties
 * you can use to render your UI.
 *
 * @param baseOptions options that will be passed into the query, supported options are listed on: https://www.apollographql.com/docs/react/api/react-hooks/#options;
 *
 * @example
 * const { data, loading, error } = useServicesQuery({
 *   variables: {
 *   },
 * });
 */
export function useServicesQuery(baseOptions?: Apollo.QueryHookOptions<ServicesQuery, ServicesQueryVariables>) {
        const options = {...defaultOptions, ...baseOptions}
        return Apollo.useQuery<ServicesQuery, ServicesQueryVariables>(ServicesDocument, options);
      }
export function useServicesLazyQuery(baseOptions?: Apollo.LazyQueryHookOptions<ServicesQuery, ServicesQueryVariables>) {
          const options = {...defaultOptions, ...baseOptions}
          return Apollo.useLazyQuery<ServicesQuery, ServicesQueryVariables>(ServicesDocument, options);
        }
export type ServicesQueryHookResult = ReturnType<typeof useServicesQuery>;
export type ServicesLazyQueryHookResult = ReturnType<typeof useServicesLazyQuery>;
export type ServicesQueryResult = Apollo.QueryResult<ServicesQuery, ServicesQueryVariables>;
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