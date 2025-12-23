/**
 * Paks Registry API Client
 *
 * A thin TypeScript client for the Paks Registry API
 */

import type {
  ListPaksQuery,
  PakWithLatestVersion,
  SearchPaksQuery,
  Pak,
  PakContentResponse,
  PakInstallResponse,
  VerifyTokenResponse,
  UserInfo,
  NotifyPublishRequest,
  NotifyPublishResponse,
  ErrorResponse,
} from './types.js';

/** Default API base URL */
export const DEFAULT_BASE_URL = 'https://api.stakpak.dev';

/** Default request timeout in milliseconds */
const DEFAULT_TIMEOUT_MS = 30_000;

/** API error types */
export type ApiErrorCode =
  | 'REQUEST_FAILED'
  | 'PARSE_ERROR'
  | 'API_ERROR'
  | 'AUTH_REQUIRED'
  | 'INVALID_TOKEN'
  | 'NOT_FOUND'
  | 'RATE_LIMITED'
  | 'VALIDATION_ERROR';

/** API error class */
export class ApiError extends Error {
  constructor(
    public readonly code: ApiErrorCode,
    message: string,
    public readonly status?: number,
    public readonly retryAfter?: number
  ) {
    super(message);
    this.name = 'ApiError';
  }
}

/** Client configuration options */
export interface PaksClientOptions {
  /** Base URL for the API (default: https://api.stakpak.dev) */
  baseUrl?: string;
  /** Request timeout in milliseconds (default: 30000) */
  timeout?: number;
  /** Authentication token */
  authToken?: string;
}

/** Paks Registry API client */
export class PaksClient {
  private baseUrl: string;
  private timeout: number;
  private authToken: string | null;

  constructor(options: PaksClientOptions = {}) {
    this.baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, '');
    this.timeout = options.timeout ?? DEFAULT_TIMEOUT_MS;
    this.authToken = options.authToken ?? null;
  }

  /** Set the authentication token */
  setToken(token: string): void {
    this.authToken = token;
  }

  /** Clear the authentication token */
  clearToken(): void {
    this.authToken = null;
  }

  /** Check if the client has an auth token set */
  isAuthenticated(): boolean {
    return this.authToken !== null;
  }

  // ========================================================================
  // Paks Endpoints
  // ========================================================================

  /** List paks with optional sorting, filtering, and pagination */
  async listPaks(query: ListPaksQuery = {}): Promise<PakWithLatestVersion[]> {
    const params = new URLSearchParams();
    if (query.sort_by) params.set('sort_by', query.sort_by);
    if (query.time_window) params.set('time_window', query.time_window);
    if (query.limit != null) params.set('limit', String(query.limit));
    if (query.offset != null) params.set('offset', String(query.offset));

    const url = `${this.baseUrl}/v1/paks${params.toString() ? `?${params}` : ''}`;
    const response = await this.request<{ results: PakWithLatestVersion[] }>(url);
    return response.results;
  }

  /** Search paks by identifier (owner/pak_name) or keywords */
  async searchPaks(query: SearchPaksQuery = {}): Promise<Pak[]> {
    const params = new URLSearchParams();
    if (query.owner) params.set('owner', query.owner);
    if (query.pak_name) params.set('pak_name', query.pak_name);
    if (query.query) params.set('query', query.query);
    if (query.limit != null) params.set('limit', String(query.limit));
    if (query.offset != null) params.set('offset', String(query.offset));

    const url = `${this.baseUrl}/v1/paks/search${params.toString() ? `?${params}` : ''}`;
    const response = await this.request<{ results: Pak[] }>(url);
    return response.results;
  }

  /**
   * Get pak content by URI
   * @param uri URI format: `owner/pak_name[@version][/path]`
   */
  async getPakContent(uri: string): Promise<PakContentResponse> {
    const encodedUri = encodeURIComponent(uri);
    const url = `${this.baseUrl}/v1/paks/content/${encodedUri}`;
    return this.request<PakContentResponse>(url);
  }

  /** Get a pak by owner and name */
  async getPak(owner: string, pakName: string): Promise<Pak | null> {
    const results = await this.searchPaks({
      owner,
      pak_name: pakName,
      limit: 1,
    });
    return results[0] ?? null;
  }

  // ========================================================================
  // Install Endpoints
  // ========================================================================

  /**
   * Get pak installation info by URI
   * @param uri URI format: `owner/pak_name[@version]`
   * 
   * This endpoint returns all metadata needed to install a pak from git,
   * and automatically records a download event.
   */
  async getPakInstall(uri: string): Promise<PakInstallResponse> {
    const encodedUri = encodeURIComponent(uri);
    const url = `${this.baseUrl}/v1/paks/install/${encodedUri}`;
    return this.request<PakInstallResponse>(url);
  }

  // ========================================================================
  // Auth Endpoints
  // ========================================================================

  /** Verify the current auth token */
  async verifyToken(): Promise<VerifyTokenResponse> {
    if (!this.isAuthenticated()) {
      throw new ApiError('AUTH_REQUIRED', 'Authentication required');
    }
    const url = `${this.baseUrl}/v1/auth/verify`;
    return this.request<VerifyTokenResponse>(url, { requireAuth: true });
  }

  /** Get current user info */
  async getCurrentUser(): Promise<UserInfo> {
    if (!this.isAuthenticated()) {
      throw new ApiError('AUTH_REQUIRED', 'Authentication required');
    }
    const url = `${this.baseUrl}/v1/auth/me`;
    return this.request<UserInfo>(url, { requireAuth: true });
  }

  // ========================================================================
  // Publish Endpoints
  // ========================================================================

  /** Notify the registry to index a new pak version */
  async notifyPublish(request: NotifyPublishRequest): Promise<NotifyPublishResponse> {
    if (!this.isAuthenticated()) {
      throw new ApiError('AUTH_REQUIRED', 'Authentication required');
    }
    const url = `${this.baseUrl}/v1/paks/index`;
    return this.request<NotifyPublishResponse>(url, {
      method: 'POST',
      body: request,
      requireAuth: true,
    });
  }

  // ========================================================================
  // Internal Helpers
  // ========================================================================

  private async request<T>(
    url: string,
    options: {
      method?: 'GET' | 'POST' | 'PUT' | 'DELETE';
      body?: unknown;
      requireAuth?: boolean;
    } = {}
  ): Promise<T> {
    const { method = 'GET', body, requireAuth = false } = options;

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Accept: 'application/json',
      'User-Agent': 'paks-api-ts/0.1.0',
    };

    if (this.authToken && (requireAuth || this.authToken)) {
      headers['Authorization'] = `Bearer ${this.authToken}`;
    }

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(url, {
        method,
        headers,
        body: body ? JSON.stringify(body) : undefined,
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (response.ok) {
        const text = await response.text();
        try {
          return JSON.parse(text) as T;
        } catch {
          throw new ApiError('PARSE_ERROR', `Failed to parse response: ${text}`);
        }
      }

      // Handle error responses
      const status = response.status;

      if (status === 401) {
        throw new ApiError('INVALID_TOKEN', 'Invalid or expired token', status);
      }

      if (status === 404) {
        throw new ApiError('NOT_FOUND', `Resource not found: ${url}`, status);
      }

      if (status === 429) {
        const retryAfter = response.headers.get('retry-after');
        throw new ApiError(
          'RATE_LIMITED',
          `Rate limited${retryAfter ? `. Retry after ${retryAfter} seconds` : ''}`,
          status,
          retryAfter ? parseInt(retryAfter, 10) : undefined
        );
      }

      // Try to parse error response
      const errorText = await response.text();
      let message = errorText;
      try {
        const errorResponse = JSON.parse(errorText) as ErrorResponse;
        message = errorResponse.error.message;
      } catch {
        // Use raw text as message
      }

      throw new ApiError('API_ERROR', `API error (${status}): ${message}`, status);
    } catch (error) {
      clearTimeout(timeoutId);

      if (error instanceof ApiError) {
        throw error;
      }

      if (error instanceof Error) {
        if (error.name === 'AbortError') {
          throw new ApiError('REQUEST_FAILED', `Request timeout after ${this.timeout}ms`);
        }
        throw new ApiError('REQUEST_FAILED', `HTTP request failed: ${error.message}`);
      }

      throw new ApiError('REQUEST_FAILED', 'Unknown error occurred');
    }
  }
}
