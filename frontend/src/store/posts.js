import { createSlice } from '@reduxjs/toolkit'
import * as api from '../api'

const postsSlice = createSlice({
  name: 'posts',
  initialState: [],
  reducers: {
    postsPush(state, action) {
      state.unshift({...action.payload, comments: []})
      return state
    },
    postsSet(state, action) {
      return action.payload;
    }
  },
})

export const { postsPush, postsSet } = postsSlice.actions

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
    dispatch(postsPush(data));
  }
  catch (error) {
    console.log(error.message)
  }
}

export default postsSlice.reducer