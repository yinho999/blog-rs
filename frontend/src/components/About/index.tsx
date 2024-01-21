import {FontAwesomeIcon} from '@fortawesome/react-fontawesome'
import AnimatedLetters from '../AnimatedLetters'
import './index.scss'
import {useState, useEffect} from 'react'
import Loader from 'react-loaders'
import {
    // faAngular,
    faCss3,
    faGitAlt,
    faHtml5,
    faJsSquare,
    faPython,
    faReact,
} from '@fortawesome/free-brands-svg-icons'

const About = () => {
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
            <div className="container about-page">
                <div className="text-zone">
                    <h1>
                        <AnimatedLetters
                            letterClass={letterClass}
                            strArray={['A', 'b', 'o', 'u', 't', ' ', 'm', 'e']}
                            index={15}
                        />
                    </h1>
                    <p>
                        I'm a very ambitious self-taught front-end developer. I have React
                        and Front-End development experience, with key strengths in
                        architecting, deploying, and maintaining front end interfaces to
                        support digital experiences.
                    </p>

                    <p>
                        I'm capable of spearheading all software development lifecycle
                        phases from requirement gathering to successful project execution.
                        Skilled at analysing results obtained from samples and making
                        recommendations for corrections to improve process efficiency.
                    </p>

                    <p>
                        If I need to define myself in one sentence that would be a great
                        team player with exceptional numerical information ,IT literacy
                        skills and remarkable critical thinking and decision-making
                        abilities.
                    </p>
                </div>

                <div className="stage-cube-cont">
                    <div className="cubespinner">
                        <div className="face1">
                            <FontAwesomeIcon icon={faJsSquare} color="#EFD81D"/>
                        </div>
                        <div className="face2">
                            <FontAwesomeIcon icon={faHtml5} color="#F06529"/>
                        </div>
                        <div className="face3">
                            <FontAwesomeIcon icon={faReact} color="#5ED4F4"/>
                        </div>
                        <div className="face4">
                            <FontAwesomeIcon icon={faCss3} color="#28A4D9"/>
                        </div>
                        <div className="face5">
                            <FontAwesomeIcon icon={faGitAlt} color="#EC4A28"/>
                        </div>
                        <div className="face6">
                            <FontAwesomeIcon icon={faPython} color="#ffde57"/>
                        </div>
                    </div>
                </div>
            </div>
            <Loader type="pacman" active/>
        </>
    )
}

export default About
