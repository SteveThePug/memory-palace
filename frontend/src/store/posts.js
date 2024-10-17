import { createSlice } from '@reduxjs/toolkit'
import * as api from '../api'

const postsSlice = createSlice({
  name: 'posts',
  initialState: [],
  reducers: {
    postsUnshift(state, action) {
      state.unshift(action.payload)
      return state
    },
    postsRemove(state, action) {
      return state.filter(post => post.post_id !== action.payload);
    },
    postsSet(state, action) {
      return action.payload;
    },
    postsUnshiftComment(state, action) {
      const comment = action.payload;
      const post = state.find(post => post.post_id === comment.post_id);
      if (post) {
        post.comments.unshift(comment);
      }
    }
  },
})

export const { postsUnshift, postsRemove, postsSet, postsUnshiftComment } = postsSlice.actions

export const postsGet = () => async(dispatch) => {
  try {
    const { data } = await api.fetchPosts();
    dispatch(postsSet(data))
}
  catch (error) {
    console.log(error.message)
  }
}

export const postsCreate = (post) => async(dispatch) => {
  try {
    console.log(post);
    const { data } = await api.createPost(post);
    dispatch(postsUnshift(data));
  }
  catch (error) {
    console.log(error.message)
  }
}

export const postsDelete = (post_id) => async(dispatch) => {
  try {
    const data = await api.deletePost(post_id);
      dispatch(postsRemove(post_id));
  }
  catch (error) {
    console.log(error.message)
  }
}

export const addComment = (commentData) => async(dispatch) => {
  try {
    const { data } = await api.addComment(commentData);
      dispatch(postsUnshiftComment(data));
  } catch (error) {
    console.log(error.message);
  }
}

export default postsSlice.reducer