import type { QueryEngineConfig } from './QueryEngine'

export type QueryEngineInstance = {
  connect(headers: string): Promise<void>
  disconnect(headers: string): Promise<void>
  /**
   * @param requestStr JSON.stringified `QueryEngineRequest | QueryEngineBatchRequest`
   * @param headersStr JSON.stringified `QueryEngineRequestHeaders`
   */
  query(requestStr: string, headersStr: string, transactionId?: string): Promise<string>
  sdlSchema(): Promise<string>
  dmmf(traceparent: string): Promise<string>
  startTransaction(options: string, traceHeaders: string): Promise<string>
  commitTransaction(id: string, traceHeaders: string): Promise<string>
  rollbackTransaction(id: string, traceHeaders: string): Promise<string>
  metrics(options: string): Promise<string>
}

export type ResultSet = {
  columns: string[]
  rows: (string)[][] // Note: we're currently stringifying any result values
}

export type Queryable = {
  queryRaw: (sql: string) => Promise<ResultSet>
  executeRaw: (sql: string) => Promise<number>
  version: () => string | undefined
  isHealthy: () => boolean
}

export type Closeable = {
  close: () => Promise<void>
}

export interface QueryEngineConstructor {
  new (config: QueryEngineConfig, logger: (log: string) => void, nodejsFnCtx?: Queryable): QueryEngineInstance
}

export interface LibraryLoader {
  loadLibrary(): Promise<Library>
}

// Main
export type Library = {
  QueryEngine: QueryEngineConstructor

  version: () => {
    // The commit hash of the engine
    commit: string
    // Currently 0.1.0 (Set in Cargo.toml)
    version: string
  }
  /**
   * This returns a string representation of `DMMF.Document`
   */
  dmmf: (datamodel: string) => Promise<string>
}
