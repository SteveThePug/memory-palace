
import { createSlice } from '@reduxjs/toolkit'
import * as api from '../api'

const usersSlice = createSlice({
  name: 'users',
  initialState: [],
  reducers: {
    usersSet(state, action) {
      return action.payload;
    }
  },
})

export const {usersSet} = usersSlice.actions

export const usersGet = () => async(dispatch) => {
  try {
    const { data } = await api.fetchUsers();
    dispatch(usersSet(data))
}
  catch (error) {
    console.log(error.message)
  }
}

export default usersSlice.reducer