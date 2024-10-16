import Posts from '../components/Posts/Posts.jsx';
import Users from '../components/Users/Users.jsx';
import Comments from '../components/Comments/Comments.jsx';

export default function Home() {
  return (
    <div>
      <Posts/>
      <Users/>
      <Comments/>
    </div>
  )
}