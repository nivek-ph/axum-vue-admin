import { http } from './http';
import { withAuthHeaders, type ApiResponse } from './core';

export interface FileRecord {
  id: number;
  name: string;
  url: string;
  tag: string;
  UpdatedAt: string;
  classId: number;
}

export interface FileListResult {
  list: FileRecord[];
  total: number;
  page: number;
  pageSize: number;
}

export interface FileFilters {
  page?: number;
  pageSize?: number;
  keyword?: string;
  classId?: number;
}

export interface CategoryRecord {
  id: number;
  name: string;
  pid: number;
  children: CategoryRecord[];
}

export function normalizeFileListResponse(payload: ApiResponse<FileListResult>): FileListResult {
  return {
    list: payload?.data?.list || [],
    total: payload?.data?.total || 0,
    page: payload?.data?.page || 1,
    pageSize: payload?.data?.pageSize || 10,
  };
}

export function normalizeCategoryListResponse(payload: ApiResponse<CategoryRecord[]>) {
  return Array.isArray(payload?.data) ? payload.data : [];
}

export async function fetchFiles(filters: FileFilters = {}) {
  const response = await http.get('/files', {
    ...withAuthHeaders(),
    params: {
      page: filters.page || 1,
      pageSize: filters.pageSize || 10,
      keyword: filters.keyword || undefined,
      classId: filters.classId,
    }
  });
  return normalizeFileListResponse(response);
}

export async function fetchCategories() {
  const response = await http.get('/attachment-categories', withAuthHeaders());
  return normalizeCategoryListResponse(response);
}

export async function saveCategory(payload: { id?: number; name: string; pid: number }) {
  return http.post(
    '/attachment-categories',
    {
      id: payload.id || 0,
      name: payload.name,
      pid: payload.pid,
    },
    withAuthHeaders()
  );
}

export async function deleteCategory(id: number) {
  return http.delete(`/attachment-categories/${id}`, withAuthHeaders());
}

export async function importFileUrl(payload: { name: string; url: string; classId?: number }) {
  return http.post('/files/import-url', payload, withAuthHeaders());
}

export async function renameFile(payload: { id: number; name: string }) {
  return http.patch(`/files/${payload.id}/name`, payload, withAuthHeaders());
}

export async function deleteFile(id: number) {
  return http.delete(`/files/${id}`, withAuthHeaders());
}

export async function uploadFile(file: File, classId?: number, onProgress?: (progress: number) => void) {
  const formData = new FormData();
  formData.append('file', file);

  const authorization = withAuthHeaders().headers.Authorization;
  const baseUrl = http.defaults.baseURL || '';
  const url = `${baseUrl}/files/upload${classId !== undefined ? `?classId=${classId}` : ''}`;

  return new Promise<any>((resolve, reject) => {
    const xhr = new XMLHttpRequest();
    xhr.open('POST', url);
    xhr.responseType = 'json';
    xhr.setRequestHeader('Authorization', authorization);

    xhr.upload.onprogress = (event) => {
      if (!onProgress || !event.lengthComputable) return;
      onProgress(Math.round((event.loaded / event.total) * 100));
    };

    xhr.onload = () => {
      resolve(xhr.response);
    };

    xhr.onerror = () => {
      reject(new Error('upload failed'));
    };

    xhr.send(formData);
  });
}
