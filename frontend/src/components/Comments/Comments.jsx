import React, { useEffect } from 'react';
import Comment from './Comment/Comment.jsx';
import { useSelector, useDispatch } from 'react-redux';
import { commentsGet } from '../../store/comments.js';

export default function Comments() {
  const dispatch = useDispatch();
  const {comments} = useSelector((state) => state);

  useEffect(() => {
    dispatch(commentsGet());
  }, [dispatch]);

  return (
    <div>
    <h1>COMMENTS</h1>
      {comments.map((commentData, index) => (
        <Comment commentData={commentData} key={index} />
      ))}
    </div>
  );
}