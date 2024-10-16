
import { createSlice } from '@reduxjs/toolkit'
import * as api from '../api'

const commentsSlice = createSlice({
  name: 'comments',
  initialState: [],
  reducers: {
    commentsSet(state, action) {
      return action.payload;
    }
  },
})

export const { commentsSet } = commentsSlice.actions

export const commentsGet = () => async (dispatch) => {
  try {
    const { data } = await api.fetchComments();
    dispatch(commentsSet(data));
  } catch (error) {
    console.error('Error fetching comments:', error.message);
  }
}

export default commentsSlice.reducer