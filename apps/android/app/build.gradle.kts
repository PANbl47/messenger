plugins {
    kotlin("jvm")
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(kotlin("stdlib"))
    testImplementation(kotlin("test"))
    testImplementation("junit:junit:4.13.2")
}

kotlin {
    jvmToolchain(21)
}

tasks.register("testDebugUnitTest") {
    group = "verification"
    description = "Alias task for early alpha JVM-backed Android checks."
    dependsOn(tasks.named("test"))
}
