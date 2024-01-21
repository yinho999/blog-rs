import './index.scss'

interface AnimatedLetters {
    letterClass: string,
    strArray: string[],
    index: number
}

const AnimatedLetters = ({letterClass, strArray, index}: AnimatedLetters) => {
    return (
        <span>
          {strArray.map((char, i) => (
              <span key={char + i} className={`${letterClass} _${i + index}`}>
                {char}
              </span>
          ))}
        </span>
    )
}

export default AnimatedLetters