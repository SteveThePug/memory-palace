import axios from 'axios';

const baseURL = 'http://localhost:8080';

const API = axios.create({baseURL});

// Add authorization token to header
API.interceptors.request.use((req) => {
  if(localStorage.getItem('profile')) {
    req.headers.Authorization = `Bearer ${JSON.parse(localStorage.getItem('profile')).token}`
  }
  return req
})

// Post APIs
export const fetchPosts = () => API.get('/posts');
export const fetchPost = (postId) => API.get(`/post/${postId}`);
export const createPost = (post) => API.post('/post', post);
export const deletePost = (postId) => API.delete(`/post/${postId}`);
export const updatePost = (postId, post) => API.patch(`/post/${postId}`, post);

// Comment APIs
export const addComment = (comment) => API.post(`/comment`, comment);
export const deleteComment = (commentId) => API.delete(`/comment/${commentId}`);
export const updateComment = (commentId, updatedComment) => API.patch(`/comment/${commentId}`, updatedComment);

// User APIs
export const fetchUsers = () => API.get('/users');
export const fetchUser = (username) => API.get(`/user/${username}`);
export const signIn = (creds) => API.post('/user/signin', creds);
export const signUp = (creds) => API.post('/user/signup', creds);
export const deleteUser = () => API.delete('/user');