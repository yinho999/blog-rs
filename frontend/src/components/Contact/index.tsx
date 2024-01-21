import Loader from 'react-loaders'
import './index.scss'
import AnimatedLetters from '../AnimatedLetters'
import {useEffect, useState} from 'react'

const Contact = () => {
    const [letterClass, setLetterClass] = useState('text-animate')

    useEffect(() => {
        const timeoutId = setTimeout(() => {
            setLetterClass('text-animate-hover')
        }, 3000)

        return () => {
            clearTimeout(timeoutId)
        }
    }, [])
    return (
        <>
            <div className="container contact-page">
                <div className="text-zone">
                    <h1>
                        <AnimatedLetters
                            letterClass={letterClass}
                            strArray={['C', 'o', 'n', 't', 'a', 'c', 't', ' ', 'm', 'e']}
                            index={15}
                        />
                    </h1>
                    <p>
                        Please feel free to contact me if you have any kind of questions or
                        requests using the form below.
                    </p>
                    {/* <div className="contact-form">
            <form>
              <ul>
                <li className="half">
                  <input type="text" name="name" placeholder="name" required />
                </li>
                <li className="half">
                  <input
                    type="email"
                    name="email"
                    placeholder="Email"
                    required
                  />
                </li>
                <li className="half">
                  <input
                    type="text"
                    name="subject"
                    placeholder="Subject"
                    required
                  />
                </li>
                <li className="half">
                  <textarea
                    placeholder="message"
                    name="message"
                    required
                  ></textarea>
                </li>
                <li className="half">
                  <input type="submit" className="flat-button" value="send" />
                </li>
              </ul>
            </form> */}
                </div>
            </div>
            {/* </div> */}
            <Loader type="pacman" active/>
        </>
    )
}

export default Contact
