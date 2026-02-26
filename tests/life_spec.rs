mod life {
    #[test]
    fn use_provided_text_file_content_for_the_initial_world() {}

    #[test]
    fn exit_with_error_when_file_does_not_exist() {}

    #[test]
    fn exit_with_error_when_file_content_is_invalid() {}

    #[test]
    fn use_random_content_when_no_file_provided() {}
}

// class LifeSpec extends AnyWordSpec with Matchers:

//   "The Life" should {

//     "use provided text file content for the initial world" in {
//       val resource = getClass.getResource("/initialWorld.life")
//       val filename = resource.toURI.getPath
//       Life.main(Array(filename))
//     }

//     "exit with error" when {
//       "the file does not exist" in {
//         Life.main(Array("noWorldFile.life"))
//       }

//       "the file content is invalid" in {
//         val resource = getClass.getResource("/invalidWorld.life")
//         val filename = resource.toURI.getPath
//         Life.main(Array(filename))
//       }
//     }

//     "use random content" when {
//       "no filename provided" in { Life.main(Array.empty) }
//     }

//   }
