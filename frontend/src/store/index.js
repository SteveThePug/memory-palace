import { configureStore } from '@reduxjs/toolkit'

import postsReducer from './posts.js'
import userReducer from './user.js'
import usersReducer from './users.js'
import commentsReducer from './comments.js'

const store = configureStore({
  reducer: {
    posts: postsReducer,
    user: userReducer,
    users: usersReducer,
    comments: commentsReducer
  },
})

export default store;