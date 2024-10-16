import { createSlice } from '@reduxjs/toolkit'
import * as api from '../api'

const userSlice = createSlice({
  name: 'user',
  initialState: null,
  reducers: {
    userSet(state, action) {
      return action.payload
    },
    userDelete(state, action) {
      return {};
    }
  },
})

export const { userSet, userDelete } = userSlice.actions

// creds looks like
// {username: '', password:'', email:''}
// data looks like
// {username: '', password:'', email:'', token:''}
export const userSignIn = (creds) => async(dispatch) => {
  try {
    const { data } = await api.signIn(creds);
    console.log(data);
    localStorage.setItem('token', JSON.stringify(data))
    dispatch(userSet(data))
  }
  catch (error) {
    console.log(error.message)
  }
}

export const userSignUp = (creds) => async(dispatch) => {
  try {
    const { data } = await api.signUp(creds);
    localStorage.setItem('token', JSON.stringify(data))
    dispatch(userSet(data))
  }
  catch (error) {
    console.log(error.message)
  }
}

export const userLogOut = () => (dispatch) => {
  //Invalidate token
  localStorage.clear('token');
  dispatch(userDelete());
}

export default userSlice.reducer