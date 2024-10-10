import { configureStore } from '@reduxjs/toolkit'

import postsReducer from './posts.js'
import userReducer from './user.js'

const store = configureStore({
  reducer: {
    posts: postsReducer,
    user: userReducer
  },
})

export default store;