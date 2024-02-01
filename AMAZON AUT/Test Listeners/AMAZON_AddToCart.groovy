import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext

class AMAZON_AddToCart {
	@BeforeTestCase
	def sampleBeforeTestCase() {
		// LOGIN TO AMAZON AUT:
		WebUI.openBrowser('')
		
		WebUI.navigateToUrl('https://www.amazon.com/')
		
		WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon.com. Spend less. Smile more/span_Hello, sign in'))
		
		WebUI.setText(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/input_email'), '9317171704')
		
		WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/inputcontinue'))
		
		WebUI.setEncryptedText(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/input_password'),
		    'VOnTHcYiLTbiLaiba9HOxQ==')
		
		WebUI.click(findTestObject('Object Repository/AMAZON_ValidateAddToCart_OR/Page_Amazon Sign-In/inputsignInSubmit'))
		
	}


	@AfterTestCase
	def sampleAfterTestCase() {
		// Closing Of Browser:
		WebUI.closeBrowser()
		

	}


	
	@BeforeTestSuite
	def sampleBeforeTestSuite() {
	}


	@AfterTestSuite
	def sampleAfterTestSuite() {
	}
}